// 10001|DEBUG|Application started

/*
    Yukarıdaki log bilgisini temsil edecek bir veri modeli tasarlıyoruz.
    pub, public anlamındadır ve ilgili enstrümana modül dışından da erişilebileceğini belirtir
*/
#[derive(Debug)] // Default formatta içeriğini yazdırmak için eklendi
pub struct SystemLog {
    pub id: u32,
    pub level: String,
    pub description: String,
}

// TODO@buraksenyurt Zaman damgası için bir alan ekleyelim

// Struct metotları impl blokların yazılır
impl SystemLog {
    /*
        SystemLog türünden bir değişkeni kolayca oluşturmak için aşağıdaki new metodu
        kullanılabilir. Argümanlardan yararlanarak yeni bir SystemLog değişkeni oluşturlur
        ve geriye döndürülür.
    */
    pub fn new(id: u32, level: String, desc: String) -> Self {
        SystemLog {
            id,
            level,
            description: desc,
        }
    }
}
