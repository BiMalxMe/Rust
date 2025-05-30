use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute, event::{self, Event, KeyCode},
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // enable raw mode and switch to alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            // split layout vertically
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0)])
                .split(f.size());

        
            let list_items = vec![
                ListItem::new("Switzerland"),
                ListItem::new("Luxembourg"),
                ListItem::new("Ireland"),
                ListItem::new("Norway"),
                ListItem::new("Singapore"),
                ListItem::new("United States"),
                ListItem::new("Denmark"),
                ListItem::new("Australia"),
                ListItem::new("Netherlands"),
                ListItem::new("Sweden"),
                ListItem::new("Canada"),
                ListItem::new("Qatar"),
                ListItem::new("Austria"),
                ListItem::new("Belgium"),
                ListItem::new("Finland"),
                ListItem::new("Germany"),
                ListItem::new("Iceland"),
                ListItem::new("France"),
                ListItem::new("United Kingdom"),
                ListItem::new("Japan"),
                ListItem::new("South Korea"),
                ListItem::new("Israel"),
                ListItem::new("New Zealand"),
                ListItem::new("Saudi Arabia"),
                ListItem::new("Italy"),
                ListItem::new("Spain"),
                ListItem::new("Kuwait"),
                ListItem::new("United Arab Emirates"),
                ListItem::new("Hong Kong"),
                ListItem::new("Taiwan"),
                ListItem::new("Brazil"),
                ListItem::new("India"),
                ListItem::new("China"),
                ListItem::new("South Africa"),
                ListItem::new("Nigeria"),
                ListItem::new("Egypt"),
                ListItem::new("Argentina"),
                ListItem::new("Mexico"),
                ListItem::new("Indonesia"),
                ListItem::new("Turkey"),
                ListItem::new("Russia"),
                ListItem::new("Poland"),
                ListItem::new("Thailand"),
                ListItem::new("Malaysia"),
                ListItem::new("Philippines"),
                ListItem::new("Vietnam"),
                ListItem::new("Colombia"),
                ListItem::new("Peru"),
                ListItem::new("Chile"),
                ListItem::new("Bangladesh"),
                ListItem::new("Pakistan"),
                ListItem::new("Ukraine"),
                ListItem::new("Romania"),
                ListItem::new("Kazakhstan"),
                ListItem::new("Algeria"),
                ListItem::new("Morocco"),
                ListItem::new("Ethiopia"),
                ListItem::new("Kenya"),
                ListItem::new("Ghana"),
                ListItem::new("Venezuela"),
                ListItem::new("Myanmar"),
                ListItem::new("Uzbekistan"),
                ListItem::new("Sri Lanka"),
                ListItem::new("Nepal"),
                ListItem::new("Cambodia"),
                ListItem::new("Ecuador"),
                ListItem::new("Bolivia"),
                ListItem::new("Paraguay"),
                ListItem::new("Uruguay"),
                ListItem::new("Cuba"),
                ListItem::new("Dominican Republic"),
                ListItem::new("Haiti"),
                ListItem::new("Guatemala"),
                ListItem::new("Honduras"),
                ListItem::new("El Salvador"),
                ListItem::new("Nicaragua"),
                ListItem::new("Costa Rica"),
                ListItem::new("Panama"),
                ListItem::new("Croatia"),
                ListItem::new("Slovakia"),
                ListItem::new("Slovenia"),
                ListItem::new("Estonia"),
                ListItem::new("Latvia"),
                ListItem::new("Lithuania"),
                ListItem::new("Bulgaria"),
                ListItem::new("Serbia"),
                ListItem::new("Hungary"),
                ListItem::new("Greece"),
                ListItem::new("Portugal"),
                ListItem::new("Czechia"),
                ListItem::new("Cyprus"),
                ListItem::new("Malta"),
                ListItem::new("Albania"),
                ListItem::new("North Macedonia"),
                ListItem::new("Bosnia and Herzegovina"),
                ListItem::new("Montenegro"),
                ListItem::new("Kosovo"),
                ListItem::new("Moldova"),
                ListItem::new("Belarus"),
                ListItem::new("Georgia"),
                ListItem::new("Armenia"),
                ListItem::new("Azerbaijan"),
                ListItem::new("Tajikistan"),
                ListItem::new("Turkmenistan"),
                ListItem::new("Kyrgyzstan"),
                ListItem::new("Afghanistan"),
                ListItem::new("Iraq"),
                ListItem::new("Iran"),
                ListItem::new("Syria"),
                ListItem::new("Yemen"),
                ListItem::new("Oman"),
                ListItem::new("Jordan"),
                ListItem::new("Lebanon"),
                ListItem::new("Palestine"),
                ListItem::new("Libya"),
                ListItem::new("Tunisia"),
                ListItem::new("Sudan"),
                ListItem::new("Somalia"),
                ListItem::new("Uganda"),
                ListItem::new("Tanzania"),
                ListItem::new("Mozambique"),
                ListItem::new("Zambia"),
                ListItem::new("Zimbabwe"),
                ListItem::new("Angola"),
                ListItem::new("Namibia"),
                ListItem::new("Botswana"),
                ListItem::new("Lesotho"),
                ListItem::new("Eswatini"),
                ListItem::new("Madagascar"),
                ListItem::new("Cameroon"),
                ListItem::new("Ivory Coast"),
                ListItem::new("Senegal"),
                ListItem::new("Mali"),
                ListItem::new("Burkina Faso"),
                ListItem::new("Niger"),
                ListItem::new("Chad"),
                ListItem::new("Central African Republic"),
                ListItem::new("Benin"),
                ListItem::new("Togo"),
                ListItem::new("Sierra Leone"),
                ListItem::new("Liberia"),
                ListItem::new("Guinea"),
                ListItem::new("Guinea-Bissau"),
                ListItem::new("Equatorial Guinea"),
                ListItem::new("Gabon"),
                ListItem::new("Republic of the Congo"),
                ListItem::new("Democratic Republic of the Congo"),
                ListItem::new("Malawi"),
                ListItem::new("South Sudan"),
                ListItem::new("Burundi"),
                ListItem::new("Rwanda"),
                ListItem::new("Djibouti"),
                ListItem::new("Eritrea"),
                ListItem::new("Mauritania"),
                ListItem::new("Cape Verde"),
                ListItem::new("Sao Tome and Principe"),
                ListItem::new("Comoros"),
                ListItem::new("Seychelles"),
                ListItem::new("Maldives"),
                ListItem::new("Bhutan"),
                ListItem::new("Brunei"),
                ListItem::new("Timor-Leste"),
                ListItem::new("Laos"),
                ListItem::new("Mongolia"),
                ListItem::new("North Korea"),
                ListItem::new("Yemen"),
                ListItem::new("Ireland"),
                ListItem::new("Greece"),
                ListItem::new("Portugal"),
                ListItem::new("Czechia"),
                ListItem::new("Cyprus"),
                ListItem::new("Malta"),
                ListItem::new("Albania"),
                ListItem::new("North Macedonia"),
                ListItem::new("Bosnia and Herzegovina"),
                ListItem::new("Montenegro"),
                ListItem::new("Kosovo"),
                ListItem::new("Moldova"),
                ListItem::new("Belarus"),
                ListItem::new("Georgia"),
                ListItem::new("Armenia"),
                ListItem::new("Azerbaijan"),
                ListItem::new("Tajikistan"),
                ListItem::new("Turkmenistan"),
                ListItem::new("Kyrgyzstan"),
                ListItem::new("Afghanistan"),
                ListItem::new("Iraq"),
                ListItem::new("Iran"),
                ListItem::new("Syria"),
                ListItem::new("Yemen"),
                ListItem::new("Oman"),
                ListItem::new("Jordan"),
                ListItem::new("Lebanon"),
                ListItem::new("Palestine"),
                ListItem::new("Libya"),
                ListItem::new("Tunisia"),
                ListItem::new("Sudan"),
                ListItem::new("Somalia"),
                ListItem::new("Uganda"),
                ListItem::new("Tanzania"),
                ListItem::new("Mozambique"),
                ListItem::new("Zambia"),
                ListItem::new("Zimbabwe"),
                ListItem::new("Angola"),
                ListItem::new("Namibia"),
                ListItem::new("Botswana"),
                ListItem::new("Lesotho"),
                ListItem::new("Eswatini"),
                ListItem::new("Madagascar"),
                ListItem::new("Cameroon"),
                ListItem::new("Ivory Coast"),
                ListItem::new("Senegal"),
                ListItem::new("Mali"),
                ListItem::new("Burkina Faso"),
                ListItem::new("Niger"),
                ListItem::new("Chad"),
                ListItem::new("Central African Republic"),
                ListItem::new("Benin"),
                ListItem::new("Togo"),
                ListItem::new("Sierra Leone"),
                ListItem::new("Liberia"),
                ListItem::new("Guinea"),
                ListItem::new("Guinea-Bissau"),
                ListItem::new("Equatorial Guinea"),
                ListItem::new("Gabon"),
                ListItem::new("Republic of the Congo"),
                ListItem::new("Democratic Republic of the Congo"),
                ListItem::new("Malawi"),
                ListItem::new("South Sudan"),
                ListItem::new("Burundi"),
                ListItem::new("Rwanda"),
                ListItem::new("Djibouti"),
                ListItem::new("Eritrea"),
                ListItem::new("Mauritania"),
                ListItem::new("Cape Verde"),
                ListItem::new("Sao Tome and Principe"),
                ListItem::new("Comoros"),
                ListItem::new("Seychelles"),
                ListItem::new("Maldives"),
                ListItem::new("Bhutan"),
                ListItem::new("Brunei"),
                ListItem::new("Timor-Leste"),
                ListItem::new("Laos"),
                ListItem::new("Mongolia"),
                ListItem::new("North Korea"),
                ListItem::new("Yemen"),
                ListItem::new("Ireland"),
                ListItem::new("Greece"),
                ListItem::new("Portugal"),
                ListItem::new("Czechia"),
                ListItem::new("Cyprus"),
                ListItem::new("Malta"),
                ListItem::new("Albania"),
                ListItem::new("North Macedonia"),
                ListItem::new("Bosnia and Herzegovina"),
                ListItem::new("Montenegro"),
                ListItem::new("Kosovo"),
                ListItem::new("Moldova"),
                ListItem::new("Belarus"),
                ListItem::new("Georgia"),
                ListItem::new("Armenia"),
                ListItem::new("Azerbaijan"),
                ListItem::new("Tajikistan"),
                ListItem::new("Turkmenistan"),
                ListItem::new("Kyrgyzstan"),
                ListItem::new("Afghanistan"),
                ListItem::new("Iraq"),
                ListItem::new("Iran"),
                ListItem::new("Syria"),
                ListItem::new("Yemen"),
                ListItem::new("Oman"),
                ListItem::new("Jordan"),
                ListItem::new("Lebanon"),
                ListItem::new("Palestine"),
                ListItem::new("Libya"),
                ListItem::new("Tunisia"),
                ListItem::new("Sudan"),
                ListItem::new("Somalia"),
                ListItem::new("Uganda"),
                ListItem::new("Tanzania"),
                ListItem::new("Mozambique"),
                ListItem::new("Zambia"),
                ListItem::new("Zimbabwe"),
                ListItem::new("Angola"),
                ListItem::new("Namibia"),
                ListItem::new("Botswana"),
                ListItem::new("Lesotho"),
                ListItem::new("Eswatini"),
                ListItem::new("Madagascar"),
                ListItem::new("Cameroon"),
                ListItem::new("Ivory Coast"),
                ListItem::new("Senegal"),
                ListItem::new("Mali"),
                ListItem::new("Burkina Faso"),
                ListItem::new("Niger"),
                ListItem::new("Chad"),
                ListItem::new("Central African Republic"),
                ListItem::new("Benin"),
                ListItem::new("Togo"),
                ListItem::new("Sierra Leone"),
                ListItem::new("Liberia"),
                ListItem::new("Guinea"),
                ListItem::new("Guinea-Bissau"),
                ListItem::new("Equatorial Guinea"),
                ListItem::new("Gabon"),
                ListItem::new("Republic of the Congo"),
                ListItem::new("Democratic Republic of the Congo"),
                ListItem::new("Malawi"),
                ListItem::new("South Sudan"),
                ListItem::new("Burundi"),
                ListItem::new("Rwanda"),
                ListItem::new("Djibouti"),
                ListItem::new("Eritrea"),
                ListItem::new("Mauritania"),
                ListItem::new("Cape Verde"),
                ListItem::new("Sao Tome and Principe"),
                ListItem::new("Comoros"),
                ListItem::new("Seychelles"),
                ListItem::new("Maldives"),
                ListItem::new("Bhutan"),
                ListItem::new("Brunei"),
                ListItem::new("Timor-Leste"),
                ListItem::new("Laos"),
                ListItem::new("Mongolia"),
                ListItem::new("North Korea"),
                ListItem::new("Yemen"),
                ListItem::new("Ireland"),
                ListItem::new("Greece"),
                ListItem::new("Portugal"),
                ListItem::new("Czechia"),
                ListItem::new("Cyprus"),
                ListItem::new("Malta"),
                ListItem::new("Albania"),
                ListItem::new("North Macedonia"),
                ListItem::new("Bosnia and Herzegovina"),
                ListItem::new("Montenegro"),
                ListItem::new("Kosovo"),
                ListItem::new("Moldova"),
                ListItem::new("Belarus"),
                ListItem::new("Georgia"),
                ListItem::new("Armenia"),
                ListItem::new("Azerbaijan"),
                ListItem::new("Tajikistan"),
                ListItem::new("Turkmenistan"),
                ListItem::new("Kyrgyzstan"),
                ListItem::new("Afghanistan"),
                ListItem::new("Iraq"),
                ListItem::new("Iran"),
                ListItem::new("Syria"),
                ListItem::new("Yemen"),
                ListItem::new("Oman"),
                ListItem::new("Jordan"),
                ListItem::new("Lebanon"),
                ListItem::new("Palestine"),
                ListItem::new("Libya"),
                ListItem::new("Tunisia"),
                ListItem::new("Sudan"),
                ListItem::new("Somalia"),
                ListItem::new("Uganda"),
                ListItem::new("Tanzania"),
                ListItem::new("Mozambique"),
                ListItem::new("Zambia"),
                ListItem::new("Zimbabwe"),
                ListItem::new("Angola"),
                ListItem::new("Namibia"),
                ListItem::new("Botswana"),
                ListItem::new("Lesotho"),
                ListItem::new("Eswatini"),
                ListItem::new("Madagascar"),
                ListItem::new("Cameroon"),
                ListItem::new("Ivory Coast"),
                ListItem::new("Senegal"),
                ListItem::new("Mali"),
                ListItem::new("Burkina Faso"),
                ListItem::new("Niger"),
                ListItem::new("Chad"),
                ListItem::new("Central African Republic"),
                ListItem::new("Benin"),
                ListItem::new("Togo"),
                ListItem::new("Sierra Leone"),
                ListItem::new("Liberia"),
                ListItem::new("Guinea"),
                ListItem::new("Guinea-Bissau"),
                ListItem::new("Equatorial Guinea"),
                ListItem::new("Gabon"),
                ListItem::new("Republic of the Congo"),
                ListItem::new("Democratic Republic of the Congo"),
                ListItem::new("Malawi"),
                ListItem::new("South Sudan"),
                ListItem::new("Burundi"),
                ListItem::new("Rwanda"),
                ListItem::new("Djibouti"),
                ListItem::new("Eritrea"),
                ListItem::new("Mauritania"),
                ListItem::new("Cape Verde"),
                ListItem::new("Sao Tome and Principe"),
                ListItem::new("Comoros"),
                ListItem::new("Seychelles"),
                ListItem::new("Maldives"),
                ListItem::new("Bhutan"),
                ListItem::new("Brunei"),
                ListItem::new("Timor-Leste"),
                ListItem::new("Laos"),
                ListItem::new("Mongolia"),
                ListItem::new("North Korea"),
                ListItem::new("Yemen"),
                ListItem::new("Ireland"),
                ListItem::new("Greece"),
                ListItem::new("Portugal"),
                ListItem::new("Czechia"),
                ListItem::new("Cyprus"),
                ListItem::new("Malta"),
                ListItem::new("Albania"),
                ListItem::new("North Macedonia"),
                ListItem::new("Bosnia and Herzegovina"),
                ListItem::new("Montenegro"),
                ListItem::new("Kosovo"),
                ListItem::new("Moldova"),
                ListItem::new("Belarus"),
                ListItem::new("Georgia"),
                ListItem::new("Armenia"),
                ListItem::new("Azerbaijan"),
                ListItem::new("Tajikistan"),
                ListItem::new("Turkmenistan"),
                ListItem::new("Kyrgyzstan"),
                ListItem::new("Afghanistan"),
                ListItem::new("Iraq"),
                ListItem::new("Iran"),
                ListItem::new("Syria"),
                ListItem::new("Yemen"),
                ListItem::new("Oman"),
                ListItem::new("Jordan"),
                ListItem::new("Lebanon"),
                ListItem::new("Palestine"),
                ListItem::new("Libya"),
                ListItem::new("Tunisia"),
                ListItem::new("Sudan"),
                ListItem::new("Somalia"),
                ListItem::new("Uganda"),
                ListItem::new("Tanzania"),
                ListItem::new("Mozambique"),
                ListItem::new("Zambia"),
                ListItem::new("Zimbabwe"),
                ListItem::new("Angola"),
                ListItem::new("Namibia"),
                ListItem::new("Botswana"),
                ListItem::new("Lesotho"),
                ListItem::new("Eswatini"),
                ListItem::new("Madagascar"),
                ListItem::new("Cameroon"),
                ListItem::new("NEpal"),];

            // build the list widget
            let list = List::new(list_items)
                .block(
                    Block::default()
                        .title("My List")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::Blue));

            // render the list
            f.render_widget(list, chunks[0]);
        })?;

        // exit on 'q'
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
