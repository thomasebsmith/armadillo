struct JSChar {
    data: u16
}

pub struct JSString {
    data: Vec<JSChar>
}
