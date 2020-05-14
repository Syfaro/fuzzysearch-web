use img_hash::{HashAlg, HasherConfig};
use yew::agent::{Agent, AgentLink, HandlerId, Job};

type Hasher = img_hash::Hasher<[u8; 8]>;

pub struct ImageHashWorker {
    link: AgentLink<Self>,
}

impl ImageHashWorker {
    fn get_hasher() -> Hasher {
        HasherConfig::with_bytes_type::<[u8; 8]>()
            .hash_alg(HashAlg::Gradient)
            .hash_size(8, 8)
            .preproc_dct()
            .to_hasher()
    }

    fn hash_image(bytes: &[u8]) -> anyhow::Result<i64> {
        let image = image::load_from_memory(&bytes)?;
        let hasher = Self::get_hasher();
        let hash = hasher.hash_image(&image);
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(hash.as_bytes());
        Ok(i64::from_be_bytes(bytes))
    }
}

impl Agent for ImageHashWorker {
    type Reach = Job<Self>;
    type Message = ();
    type Input = Vec<u8>;
    type Output = anyhow::Result<i64>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, bytes: Self::Input, who: HandlerId) {
        self.link.respond(who, Self::hash_image(&bytes))
    }
}
