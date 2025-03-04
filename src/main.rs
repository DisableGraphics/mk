use std::{env::args, error::Error, io::ErrorKind, path::Path};

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = args().collect();
	if args.len() < 2 {
		eprintln!("Usage: {} file1... dest", args[0]);
		return Ok(());
	}

	let files = &args[1..args.len()-1];
	let dest = args.last().unwrap().to_owned();

	let res = std::fs::create_dir_all(&dest);
	if let Err(e) = res {
		if e.kind() != ErrorKind::AlreadyExists {
			return Err(Box::new(e));
		}
	}

	let path = Path::new(&dest);

	for file in files {
		let newfile = path.join(file);
		std::fs::rename(file, newfile)?;
	}

	Ok(())
}
