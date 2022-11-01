mod test;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut is_constructable = true;

    let ransom_note = ransom_note[..].chars().collect::<Vec<char>>();
    let mut magazine = magazine[..].chars().collect::<Vec<char>>();

    for item in ransom_note.iter() {
        let index = magazine.iter().position(|char| char == item);

        if let Some(index) = index {
            magazine.remove(index);
        } else {
            is_constructable = false;
            break;
        }
    }

    is_constructable
}

fn main() {}
