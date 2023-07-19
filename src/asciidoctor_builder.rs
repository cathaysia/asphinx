use tokio::process;

pub struct AsciidoctorBuilder {
    input_file: String,
    destination_dir: String,
    attrs: Vec<String>,
    plugins: Vec<String>,
}

impl AsciidoctorBuilder {
    pub fn new(input_file: String, destination_dir: String) -> Self {
        Self {
            input_file,
            destination_dir,
            attrs: Default::default(),
            plugins: Default::default(),
        }
    }

    pub fn attr(self: &mut Self, attr: String) -> &mut Self {
        self.attrs.push(attr);
        self
    }

    pub fn plugin(self: &mut Self, plugin: String) -> &mut Self {
        if plugin == "asciidoctor-diagram" {
            self.attrs.push(format!("outdir={}", self.destination_dir));
        }
        self.plugins.push(plugin);
        self
    }

    pub async fn build(self: &Self) -> String {
        let mut cmd_ = process::Command::new("asciidoctor");
        let cmd = cmd_
            .arg(&self.input_file)
            .arg("-D")
            .arg(&self.destination_dir)
            .arg("-o")
            .arg("-");

        for attr in &self.attrs {
            cmd.arg("-a").arg(&attr);
        }
        for plugin in &self.plugins {
            cmd.arg("-r").arg(plugin);
        }

        let res = cmd.output().await.unwrap();
        String::from_utf8_lossy(&res.stdout).to_string()
    }
}
