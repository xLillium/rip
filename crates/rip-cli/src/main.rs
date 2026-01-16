use std::io::{self, Write};

use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_eventsource::{Error as EventSourceError, Event, RequestBuilderExt};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "rip")]
#[command(about = "RIP CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        prompt: String,
        #[arg(long, default_value = "http://127.0.0.1:7341")]
        server: String,
        #[arg(
            long,
            default_value_t = true,
            value_parser = clap::value_parser!(bool),
            action = clap::ArgAction::Set
        )]
        headless: bool,
    },
}

#[derive(Deserialize)]
struct SessionCreated {
    session_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run(Cli::parse()).await
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Run {
            prompt,
            server,
            headless,
        } => {
            if !headless {
                eprintln!("interactive mode not implemented; falling back to headless");
            }
            run_headless(prompt, server).await?;
        }
    }

    Ok(())
}

async fn run_headless(prompt: String, server: String) -> anyhow::Result<()> {
    let client = Client::new();
    let session_id = create_session(&client, &server).await?;
    send_input(&client, &server, &session_id, &prompt).await?;
    stream_events(&client, &server, &session_id).await?;
    Ok(())
}

async fn create_session(client: &Client, server: &str) -> anyhow::Result<String> {
    let url = format!("{server}/sessions");
    let response = client.post(url).send().await?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("create session failed: {status}");
    }
    let payload: SessionCreated = response.json().await?;
    Ok(payload.session_id)
}

async fn send_input(
    client: &Client,
    server: &str,
    session_id: &str,
    input: &str,
) -> anyhow::Result<()> {
    let url = format!("{server}/sessions/{session_id}/input");
    let response = client
        .post(url)
        .json(&serde_json::json!({ "input": input }))
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("send input failed: {status}");
    }
    Ok(())
}

async fn stream_events(client: &Client, server: &str, session_id: &str) -> anyhow::Result<()> {
    let url = format!("{server}/sessions/{session_id}/events");
    let mut stream = client.get(url).eventsource()?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    while let Some(next) = stream.next().await {
        match next {
            Ok(Event::Open) => {}
            Ok(Event::Message(msg)) => {
                writeln!(handle, "{}", msg.data)?;
                handle.flush()?;
            }
            Err(EventSourceError::StreamEnded) => break,
            Err(err) => return Err(err.into()),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::Method::{GET, POST};
    use httpmock::MockServer;

    #[tokio::test]
    async fn create_session_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/sessions");
            then.status(201)
                .header("content-type", "application/json")
                .body(r#"{"session_id":"abc"}"#);
        });

        let client = Client::new();
        let session_id = create_session(&client, &server.base_url()).await.unwrap();
        assert_eq!(session_id, "abc");
        mock.assert();
    }

    #[tokio::test]
    async fn create_session_failure() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(POST).path("/sessions");
            then.status(500);
        });

        let client = Client::new();
        let err = create_session(&client, &server.base_url())
            .await
            .unwrap_err();
        assert!(err.to_string().contains("create session failed"));
    }

    #[tokio::test]
    async fn send_input_failure() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(POST).path("/sessions/s1/input");
            then.status(400);
        });

        let client = Client::new();
        let err = send_input(&client, &server.base_url(), "s1", "hi")
            .await
            .unwrap_err();
        assert!(err.to_string().contains("send input failed"));
    }

    #[tokio::test]
    async fn stream_events_reads_messages() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/sessions/s1/events");
            then.status(200)
                .header("content-type", "text/event-stream")
                .body("data: {\"type\":\"session_started\"}\n\n");
        });
        let client = Client::new();
        let result = stream_events(&client, &server.base_url(), "s1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn run_headless_with_interactive_flag() {
        let server = MockServer::start();
        let _create = server.mock(|when, then| {
            when.method(POST).path("/sessions");
            then.status(201)
                .header("content-type", "application/json")
                .body(r#"{"session_id":"abc"}"#);
        });
        let _input = server.mock(|when, then| {
            when.method(POST).path("/sessions/abc/input");
            then.status(202);
        });
        let _events = server.mock(|when, then| {
            when.method(GET).path("/sessions/abc/events");
            then.status(200)
                .header("content-type", "text/event-stream")
                .body("data: {\"type\":\"session_started\"}\n\n");
        });

        let cli = Cli {
            command: Commands::Run {
                prompt: "hello".to_string(),
                server: server.base_url(),
                headless: false,
            },
        };
        let result = run(cli).await;
        assert!(result.is_ok());
    }

    #[test]
    fn cli_parses_run() {
        let cli = Cli::parse_from(["rip", "run", "hello"]);
        match cli.command {
            Commands::Run { prompt, .. } => assert_eq!(prompt, "hello"),
        }
    }

    #[test]
    fn cli_defaults_headless() {
        let cli = Cli::parse_from(["rip", "run", "hello"]);
        match cli.command {
            Commands::Run { headless, .. } => assert!(headless),
        }
    }

    #[test]
    fn cli_respects_server_flag() {
        let cli = Cli::parse_from(["rip", "run", "hello", "--server", "http://local"]);
        match cli.command {
            Commands::Run { server, .. } => assert_eq!(server, "http://local"),
        }
    }

    #[test]
    fn cli_respects_headless_flag() {
        let cli = Cli::parse_from(["rip", "run", "hello", "--headless", "false"]);
        match cli.command {
            Commands::Run { headless, .. } => assert!(!headless),
        }
    }
}
