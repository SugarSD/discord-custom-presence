use std::error::Error;
use iced::widget::{button, column, text, text_input, pick_list, Column};
use discord_rich_presence::{activity::{self, ActivityType, Assets}, DiscordIpc, DiscordIpcClient};

#[derive(Default)]
struct App {
    user_activity_option: Option<ActivityOptions>,
    activity_type: Option<ActivityType>,
    activity_main_text: String,
    activity_secondary_text: String,
    main_image_url: String,
    client: Option<DiscordIpcClient>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityOptions {
    Playing,
    Listening,
    Watching,
    Competing,
}

#[derive(Debug, Clone)]
pub enum Message {
    ActivitySelected(ActivityOptions),
    MainTextChanged(String),
    SecondaryTextChanged(String),
    MainImageChanged(String),
    ButtonPressed,
}

impl App {

    pub fn init_presence (&mut self) {
        let mut client = DiscordIpcClient::new("1374549527650107494").unwrap();
        let activity_type = self.activity_type.take().unwrap();
        let state = self.activity_main_text.as_str();
        let details = self.activity_secondary_text.as_str();
        let large_image = self.main_image_url.as_str();

        println!("{}", self.activity_main_text);
        client.connect().expect("Failed to connect!");
        println!("{client:?}");
        client.set_activity(activity::Activity::new()
            .assets(Assets::new()
                .large_image(large_image)
            )
            .state(state)
            .details(details)
            .activity_type(activity_type.clone())
        ).expect("Failed to Set Activity!");
        println!("{client:?}");
        self.activity_type = Some(activity_type);
        self.client = Some(client);
    }

    pub fn update_presence (&mut self) {
        let mut client = self.client.take().unwrap();
        let activity_type = self.activity_type.take().unwrap();
        let state = self.activity_main_text.as_str();
        let details = self.activity_secondary_text.as_str();
        let large_image = self.main_image_url.as_str();

        println!("{}", self.activity_main_text);
        let _ = client.close();
        client.reconnect().expect("Failed to connect!");
        println!("{client:?}");
        client.set_activity(activity::Activity::new()
            .assets(Assets::new()
                .large_image(large_image)
            )
            .state(state)
            .details(details)
            .activity_type(activity_type.clone())
        ).expect("Failed to Set Activity!");
        println!("{client:?}");
        self.activity_type = Some(activity_type);
        self.client = Some(client);
    }

    pub fn view(&self) -> Column<Message> {
        let activity_options = [
            ActivityOptions::Playing,
            ActivityOptions::Listening,
            ActivityOptions::Watching,
            ActivityOptions::Competing,
        ];
        column![
            text_input("Primary image URL/Asset name...", &*self.main_image_url)
            .on_input(Message::MainImageChanged),
            text_input("Primary activity text...", &*self.activity_main_text)
            .on_input(Message::MainTextChanged),
            text_input("Secondary activity text...", &*self.activity_secondary_text)
            .on_input(Message::SecondaryTextChanged),
            pick_list(
                activity_options,
                self.user_activity_option,
                Message::ActivitySelected
            )
            .placeholder("Activity Type"),
            button("Set Presence").on_press(Message::ButtonPressed)
        ]
    }
    
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ActivitySelected(activity) => {
                self.user_activity_option = Some(activity);
                match activity {
                    ActivityOptions::Playing => {
                        self.activity_type = Some(ActivityType::Playing);
                    }
                    ActivityOptions::Listening => {
                        self.activity_type = Some(ActivityType::Listening);
                    }
                    ActivityOptions::Watching => {
                        self.activity_type = Some(ActivityType::Watching);
                    }
                    ActivityOptions::Competing => {
                        self.activity_type = Some(ActivityType::Competing);
                    }
                }
            }
            Message::MainTextChanged(content) => {
                self.activity_main_text = content;
            }
            Message::SecondaryTextChanged(content) => {
                self.activity_secondary_text = content;
            }
            Message::MainImageChanged(content) => {
                self.main_image_url = content
            }
            Message::ButtonPressed => {
                if self.client.is_some() {
                    self.update_presence();
                } else {
                    self.init_presence();
                }
            }
        }
    }
}

impl std::fmt::Display for ActivityOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Playing => "Playing",
            Self::Listening => "Listening",
            Self::Watching => "Watching",
            Self::Competing => "Competing",
        })
    }
}


fn main() -> iced::Result {
    iced::run("Discord Presence Customizer", App::update, App::view)
}