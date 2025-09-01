pub trait RenderScheme {
    /// Render tables to template
    /// returns Vec<(table_name, rendered_content)>
    fn render_tables(&self, registry: &handlebars::Handlebars, template: &str) -> anyhow::Result<Vec<(String, String)>>;
    fn render(&self, registry: &handlebars::Handlebars, template: &str) -> anyhow::Result<String>;
}