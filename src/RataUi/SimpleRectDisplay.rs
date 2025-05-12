use ratatui::{
    Terminal,
    backend::TestBackend,
    widgets::{Block, Borders},
};

fn main() {
    // Initailizing the terminal with width and height
    let backend = TestBackend::new(10, 10);

    // initializing the instance such that all modification happens in this instance
    let mut terminal = Terminal::new(backend).unwrap();

    // pass the frame as a closure and frame is automatically created
    // We can rennder only one frame at a time
    // if we use multiple draw then only one can be printed
    // as one is flused and cleared after another printed
    // if multiple then last frame can only be printed
    terminal
        .draw(|f| {
            //get the size i decalre earlier
            let size = f.size();

            //write into the terminal
            let block = Block::default().title("Hello UI").borders(Borders::ALL);
            //render into the terminal
            f.render_widget(block, size);
        })
        .unwrap();

    //finally buffer is used to diaplay thw whole fake terminal
    let buffer = terminal.backend().buffer();
    println!("{:?}", buffer)
}
