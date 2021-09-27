use std::time::Duration;

use async_trait::async_trait;
use chrono::Local;
use tokio::{sync::mpsc::{channel, Receiver, Sender}, time::sleep};

pub type SectionId = usize;

#[derive(Debug)]
pub struct Message(SectionId, String);

pub struct Renderer {
    renderer: Sender<Message>,
    messages: Receiver<Message>,
    sections: Vec<String>,
}

impl Renderer {
    pub fn new(buffer: usize) -> Self {
        let (renderer, messages) = channel(buffer);
        Renderer {
            renderer,
            messages,
            sections: Vec::new(),
        }
    }

    pub fn add_section<S>(&mut self, initial: &str, section: S)
    where
        S: 'static + Section + Send + Sync,
    {
        self.sections.push(initial.into());

        let id_ = self.sections.len() - 1;
        let renderer_ = self.renderer.clone();

        tokio::spawn(async move {
            section.start(id_, renderer_).await;
        });
    }

    pub async fn start(&mut self, separator: &str) {
        println!("{}", self.sections.join(separator));
        while let Some(Message(i, t)) = self.messages.recv().await {
            self.sections[i] = t;
            println!("{}", self.sections.join(separator));
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        let (renderer, messages) = channel(32);
        Renderer {
            renderer,
            messages,
            sections: Vec::new(),
        }
    }
}

#[async_trait]
pub trait Section {
    async fn start(&self, id: SectionId, renderer: Sender<Message>);
}

pub struct Static;

#[async_trait]
impl Section for Static {
    async fn start(&self, _: SectionId, _: Sender<Message>) {

    }
}

pub struct Time {
    format: String,
    delay: Duration,
}

impl Time {
    fn new(format: &str, delay: Duration) -> Self {
        Time {
            format: format.into(),
            delay,
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            format: "%Y-%m-%d %H:%M:%S".into(),
            delay: Duration::from_secs(1),
        }
    }
}

#[async_trait]
impl Section for Time {
    async fn start(&self, id: SectionId, renderer: Sender<Message>) {
        loop {
            let m = Local::now().format(&self.format);
            renderer.send(Message(id, m.to_string())).await.unwrap();
            sleep(self.delay).await;
        }
    }
}
