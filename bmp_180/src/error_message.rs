use nano::drivers::twi::StatusCode;

pub struct ErrorMessage {}
impl ErrorMessage {
    pub fn into_message(status_code: u8) -> &'static str {
        match status_code {
            StatusCode::STARTED => "Успешно отправлен первый сигнал СТАРТ",
            StatusCode::SLA_W_ACK => {
                "Успешно принял адрес в режиме записи и ответил подтверждением"
            }
            StatusCode::DATA_W_ACK => "Датчик принял адрес регистра и подтвердил его",
            StatusCode::START_REPEATED => {
                "Успешно отправлен повторный сигнал СТАРТ для переключения в режим чтения"
            }
            _ => "",
        }
    }
}
