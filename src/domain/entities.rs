pub struct PokemonNumber(u16);

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 899 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(n: PokemonNumber) -> Self {
        n.0
    }
}

pub struct PokemonName(String);

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

enum PokemonType {
    Eletric,
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(t: String) -> Result<Self, Self::Error> {
        match t.as_str() {
            "Electric" => Ok(Self::Eletric),
            _ => Err(()),
        }
    }
}

pub struct PokemonTypes(Vec<PokemonType>);

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(types: Vec<String>) -> Result<Self, Self::Error> {
        if types.is_empty() {
            Err(())
        } else {
            let mut pokemon_types = vec![];
            for t in types.iter() {
                match PokemonType::try_from(String::from(t)) {
                    Ok(pokemon_type) => pokemon_types.push(pokemon_type),
                    _ => return Err(()),
                }
            }
            Ok(Self(pokemon_types))
        }
    }
}
