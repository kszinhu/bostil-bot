use serenity::{
    builder::CreateButton,
    model::prelude::{component::ButtonStyle, ReactionType},
};

pub struct Button {
    name: String,
    emoji: Option<ReactionType>,
    label: String,
    style: ButtonStyle,
}

impl Button {
    pub fn new(name: &str, label: &str, style: ButtonStyle, emoji: Option<ReactionType>) -> Self {
        Self {
            emoji,
            style,
            name: name.to_string(),
            label: label.to_string(),
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn create(&self) -> CreateButton {
        let mut b = CreateButton::default();

        b.custom_id(&self.name);
        b.label(&self.label);
        b.style(self.style.clone());

        if let Some(emoji) = &self.emoji {
            b.emoji(emoji.clone());
        }

        b
    }
}
