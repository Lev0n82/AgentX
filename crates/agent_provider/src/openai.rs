#![warn(unused_variables)]
use std::error::Error;

use tracing::{debug, info, warn, error};

use async_trait::async_trait;
use futures::StreamExt;

// use async_openai::config::OpenA/IConfig;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};

// use agent_schema::Message;
use crate::llmbase::LLMBase;

#[derive(Debug)]
pub struct OpenAIGPTAPI {
    client: Client<OpenAIConfig>,
}
#[async_trait]
impl LLMBase for OpenAIGPTAPI {
    // async fn acompletion(&self) -> String {

    // }
    // async fn ask(&self, msg: &Message) -> String {

    // }
    async fn aask(&self, msg: String) -> String {
        // debug!("OpenAIGPTAPI msg: {}", msg);
        debug!("chat with OpenAIGPTAPI...");
        let data = self.aask(&msg).await.expect("msg is not a message");
        data
    }
}

impl Default for OpenAIGPTAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAIGPTAPI {
    pub fn new() -> Self {
        // Configure client to use custom base URL if OPENAI_API_BASE is set
        let config = if let Ok(base_url) = std::env::var("OPENAI_API_BASE") {
            if !base_url.is_empty() {
                info!("[OLLAMA DEBUG] Configuring client with custom base URL: {}", base_url);
                OpenAIConfig::new().with_api_base(base_url)
            } else {
                info!("[OLLAMA DEBUG] Using default OpenAI configuration");
                OpenAIConfig::new()
            }
        } else {
            info!("[OLLAMA DEBUG] OPENAI_API_BASE not set, using default OpenAI configuration");
            OpenAIConfig::new()
        };
        
        let client = Client::with_config(config);
        Self { client }
    }

    pub async fn aask_with_role(
        &self,
        messages: Vec<ChatCompletionRequestMessage>,
    ) -> Result<String, Box<dyn Error>> {
        let model = std::env::var("OPENAI_API_MODEL").unwrap_or("gpt-3.5-turbo".to_string());
        info!("[OLLAMA DEBUG] Using model: {}", model);
        info!("[OLLAMA DEBUG] API Base: {:?}", std::env::var("OPENAI_API_BASE"));
        
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            // .max_tokens(4096u16)
            .messages(messages)
            .build()?;

        info!("[OLLAMA DEBUG] Request created, starting stream...");
        let mut stream = self.client.chat().create_stream(request).await?;
        info!("[OLLAMA DEBUG] Stream initialized");

        // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
        //
        //  Note that stdout is frequently line-buffered by default so it may be necessary
        //  to use io::stdout().flush() to ensure the output is emitted immediately.
        //
        //  The print! macro will lock the standard output on each call.
        //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
        //  To avoid this, lock stdout with io::stdout().lock():

        // let mut lock = stdout().lock();
        let mut rsp = String::new();
        let mut chunk_count = 0;
        while let Some(result) = stream.next().await {
            chunk_count += 1;
            match result {
                Ok(response) => {
                    info!("[OLLAMA DEBUG] Chunk {}: choices.len={}", chunk_count, response.choices.len());
                    response.choices.iter().enumerate().for_each(|(idx, chat_choice)| {
                        info!("[OLLAMA DEBUG] Choice {}: delta.content={:?}, delta.role={:?}, finish_reason={:?}", 
                            idx, chat_choice.delta.content, chat_choice.delta.role, chat_choice.finish_reason);
                        if let Some(ref content) = chat_choice.delta.content {
                            info!("[OLLAMA DEBUG] Adding content: {:?}", content);
                            rsp += content
                        } else {
                            warn!("[OLLAMA DEBUG] No content in delta!");
                        }
                    });
                }
                Err(err) => {
                    error!("[OLLAMA DEBUG] Stream error: {:?}", err);
                }
            }
            // stdout().flush()?;
        }

        info!("[OLLAMA DEBUG] Stream complete. Total chunks: {}, Response length: {}", chunk_count, rsp.len());
        debug!("aask_with_role:\n {:?}", rsp);

        Ok(rsp)
    }

    pub async fn aask(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let model = std::env::var("OPENAI_API_MODEL").unwrap_or("gpt-3.5-turbo".to_string());
        info!("[OLLAMA DEBUG] aask - Using model: {}", model);
        info!("[OLLAMA DEBUG] aask - Prompt length: {} chars", content.len());
        
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            // .max_tokens(4096u16)
            .messages([ChatCompletionRequestMessageArgs::default()
                .content(content)
                .role(Role::User)
                .build()?])
            .build()?;

        info!("[OLLAMA DEBUG] aask - Starting stream...");
        let mut stream = self.client.chat().create_stream(request).await?;
        info!("[OLLAMA DEBUG] aask - Stream initialized");

        // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
        //
        //  Note that stdout is frequently line-buffered by default so it may be necessary
        //  to use io::stdout().flush() to ensure the output is emitted immediately.
        //
        //  The print! macro will lock the standard output on each call.
        //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
        //  To avoid this, lock stdout with io::stdout().lock():

        // let mut lock = stdout().lock();
        let mut rsp = String::new();
        let mut chunk_count = 0;
        while let Some(result) = stream.next().await {
            chunk_count += 1;
            match result {
                Ok(response) => {
                    info!("[OLLAMA DEBUG] aask Chunk {}: choices.len={}", chunk_count, response.choices.len());
                    response.choices.iter().enumerate().for_each(|(idx, chat_choice)| {
                        info!("[OLLAMA DEBUG] aask Choice {}: delta.content={:?}, delta.role={:?}, finish_reason={:?}", 
                            idx, chat_choice.delta.content, chat_choice.delta.role, chat_choice.finish_reason);
                        if let Some(ref content) = chat_choice.delta.content {
                            info!("[OLLAMA DEBUG] aask Adding content chunk: {} bytes", content.len());
                            rsp += content
                        } else {
                            warn!("[OLLAMA DEBUG] aask No content in delta!");
                        }
                    });
                }
                Err(err) => {
                    error!("[OLLAMA DEBUG] aask Stream error: {:?}", err);
                }
            }
            // stdout().flush()?;
        }

        info!("[OLLAMA DEBUG] aask Stream complete. Total chunks: {}, Response length: {}", chunk_count, rsp.len());
        info!("rsp: {:?}", rsp);

        Ok(rsp)
    }
}
