mod oxide;

#[tokio::main]
async fn main() {
    let mut renderer = oxide::Renderer::default();

    renderer.add_section("pure@linux", oxide::Static);
    renderer.add_section("...", oxide::Time::default());

    renderer.start(" | ").await;
}
