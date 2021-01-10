pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    uniform mat4 uTransfrom;
    void main() {
        gl_Position = uTransfrom * aPosition;
    }
"#;
