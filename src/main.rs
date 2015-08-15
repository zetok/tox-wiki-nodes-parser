/*
    Copyright © 2015 Zetok Zalbavar <zetok@openmailbox.org>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/


use std::io::Read;
use std::fs::File;


/*
    Function to read file and return vector of strings, each of them
    corresponding to a line from a file.

    In a case where there is no file, return early.
*/
fn vec_strings(file: &str) -> Result<Vec<String>, ()> {
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening {}: {}", file, e);
            return Err(())
        },
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    Ok(content.lines().map(|l| l.to_string()).collect())
}

fn parse_into_string(string: &str, ipv6: bool) -> Option<String> {
    if string.starts_with('|') &&
            (string.ends_with("|") || string.ends_with("|\r")) {
        let split: Vec<&str> = string.split_terminator("| ").collect();
        // return early if split stuff doesn't appear to match
        // list of nodes
        if split.len() < 8 {
            return None
        }

        let mut ret_str: String = String::new();

        // if no IPv6, push only IPv4 to string
        if !ipv6 {
            ret_str.push_str(split[1]);
        } else {
            // if there is IPv6, push it instead
            ret_str.push_str(split[2]);
        }

        // push port, public key and maintainer
        for s in 3..6 {
            ret_str.push_str(split[s]);
        }

        // push port to string
        //ret_str.push_str(split[3]);

        // and push public key
        //ret_str.push_str(split[4]);

        // and push maintainer
        //ret_str.push_str(split[5]);

        // return that string
        return Some(ret_str)
    }
    None
}


fn main() {
    let file = "nodes.txt";
    let vecs = match vec_strings(file) {
        Ok(v) => v,
        Err(_) => return,
    };

    for s in vecs {
        // if has IPv6, print it first
        if !s.contains("NONE") {
            if let Some(node) = parse_into_string(&s, true) {
                println!("{}", node);
            }
        }
        if let Some(node) = parse_into_string(&s, false) {
            println!("{}", node);
        }
    }
}
