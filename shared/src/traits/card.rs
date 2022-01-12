use crate::ui::card;
pub trait Card{
    fn to_card(&self)->card::Card;
}