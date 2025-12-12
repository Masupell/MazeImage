use image_manipulation::image_shader::image_shader;

pub fn run()
{
    let input = image::open("src/res/546727.jpg");
    if input.is_err()
    {
        println!("Error\n(Could be wrong path/does not exist");
        return;
    }
    let mut image = input.unwrap();
    image = image.blur(5.0);
    let output = pollster::block_on(image_shader(image, "src/sobel_operator.wgsl"));
    output.save("src/res/output.png").unwrap();
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test() {
        run();
    }
}