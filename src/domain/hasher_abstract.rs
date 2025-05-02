pub trait HasherAbstract {
    async fn hash(&self, password: String) -> anyhow::Result<String>;
    async fn verify(&self, password: String, hash: String) -> anyhow::Result<bool>;
}
