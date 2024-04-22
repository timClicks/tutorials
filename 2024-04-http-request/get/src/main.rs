fn main() -> Result<(), ureq::Error> {
	let body = ureq::get("https://timclicks.dev")
		.call()?
		.into_string()?;
		
    println!("{body}");

    Ok(())
}
