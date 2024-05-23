pub struct Context {
    messages: MessageHistory,
    current_message: Option<Message>,
}
impl Context {
    pub fn new() -> Self {
        Self {
            messages: MessageHistory::new(),
            current_message: None,
        }
    }

    pub fn ask(&mut self, question: String) {
        // todo: build context from last messages
        // todo: maybe have llm rank and summarize lasr messages?
        if let Some(message) = self.current_message.take() {
            self.messages.0.push(message);
        }
        self.current_message = Some(Message {
            question,
            answer: None,
        });
    }

    pub fn get_answer(&mut self) -> Option<String> {
        // todo: poll for answer
        match &self.current_message {
            Some(message) => message.answer.clone(),
            None => None,
        }
    }
}
pub struct MessageHistory(Vec<Message>);
impl MessageHistory {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    question: String,
    answer: Option<String>,
}
impl Message {
    pub fn new(question: String) -> Self {
        Self {
            question,
            answer: None,
        }
    }
}
