use super::entities::{PokemonName, PokemonNumber, PokemonTypes};
use crate::repositories::pokemon::{Insert, Repository};

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

fn execute(repo: &mut dyn Repository, req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
            Insert::Ok(number) => Response::Ok(u16::from(number)),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use crate::repositories::pokemon::InMemoryRepository;

    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let mut repo = InMemoryRepository::new();
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(&mut repo, req);
        match res {
            Response::Ok(res_number) => assert_eq!(res_number, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let mut repo = InMemoryRepository::new();
        let req = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let res = execute(&mut repo, req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();
        let mut repo = InMemoryRepository::new();
        repo.insert(number, name, types);
        let req = Request {
            number: 25,
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };

        let res = execute(&mut repo, req);

        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let mut repo = InMemoryRepository::new().with_error();
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(&mut repo, req);

        match res {
            Response::Error => {}
            _ => unreachable!(),
        }
    }
}
