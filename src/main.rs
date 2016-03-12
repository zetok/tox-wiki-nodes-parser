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


// For fetching from wiki
extern crate hyper;

use hyper::Client;
use hyper::header::Connection;

use std::io::Read;

/*
    Function to download list of nodes from wiki and return vector of
    strings, each string corresponding to the node from wiki.
*/
fn dl_vec(link: &str) -> Vec<String> {
    let mut body = String::new();
    let client = Client::new();

    client.get(link)
        // set a header and send it
        .header(Connection::close())
        .send().unwrap()
        // read the response
        .read_to_string(&mut body).unwrap();

    body.lines().map(|l| l.to_string()).collect()
}

// example of a string to parse:
// | 144.76.60.215           | 2a01:4f8:191:64d6::1                   | 33445  | 04119E835DF3E78BACF0F84235B300546AF8B936F035185E2A8E9E0A67C8924F  | sonOfRa          | DE        |
fn parse_into_string(string: &str, ipv6: bool) -> Option<String> {
    if string.starts_with('|') &&
            (string.ends_with("|") || string.ends_with("|\r")) {
        let split: Vec<&str> = string.split_terminator("| ").collect();
        // return early if split stuff doesn't appear to match
        // list of nodes
        if split.len() < 6 {
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
    let vecs = dl_vec("https://wiki.tox.chat/users/nodes?do=edit");

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
