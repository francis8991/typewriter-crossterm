use rand::prelude::*;
#[derive(Debug)]
pub struct WordSelector {
    pub words: Vec<String>,
}

impl WordSelector {
    // pub fn new(num_words: usize) -> Result<Self, io::Error> {
    //     match wordlists::get_word_list("top250") {
    //         Some(str) => {
    //             let mut words = str.split('\n').collect::<Vec<&str>>();
    //             let mut rng = rand::thread_rng();

    //             words.shuffle(&mut rng);
    //             let args = TypeConfig::parse();
    //             Ok(Self {
    //                 words: &words[..args.num_words],
    //             })
    //         }
    //         None => Err("Something Error"),
    //     }
    // }
    pub fn from_string(word_list: &str) -> Self {
        Self {
            words: word_list
                .split('\n')
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        }
    }

    pub fn new_words(&mut self, num_words: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        self.words.shuffle(&mut rng);
        self.words[..num_words].to_vec()
    }
}
