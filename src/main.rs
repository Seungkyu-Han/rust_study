#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

#[derive(Debug, PartialEq, Eq)]
struct TVChannel {
    name: String,
    channel_type: ChannelType,
}

fn main() {
    let channels = [
        TVChannel {
            name: String::from("CS"),
            channel_type: ChannelType::Comedy,
        },
        TVChannel {
            name: String::from("CS"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TVChannel {
            name: String::from("CS"),
            channel_type: ChannelType::News,
        },
    ];

    println!("{channels:?}");
}
