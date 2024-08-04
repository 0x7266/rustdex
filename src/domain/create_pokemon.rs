use super::entities::{PokemonName, PokemonNumber, PokemonTypes};

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
}

fn execute(r: Request) -> Response {
    match (
        PokemonNumber::try_from(r.number),
        PokemonName::try_from(r.name),
        PokemonTypes::try_from(r.types),
    ) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(u16::from(number)),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(req);
        match res {
            Response::Ok(res_number) => assert_eq!(res_number, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let res = execute(req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }
}
