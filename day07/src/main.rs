use util::read_input;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {

}
