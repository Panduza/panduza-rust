



struct PzaTopic {

    topic: String,

}

impl PzaTopic {

    pub fn new(topic: String) -> Self
    {
        Self
        {
            topic: topic
        }
    }

    pub fn get_topic(&self) -> String
    {
        self.topic.clone()
    }

}