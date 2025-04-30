# Ders 10: Smart Pointers

Verinin bellekteki temsili sırasında şartlara göre farklı senaryolar için farklı enstrümanlar kullanılabilir. Bu nedenle
**Smart Pointer** kullanımını bilmek önemlidir. Smart Pointer türlerine geçmeden önce bazı temek kavramlar üzerinde
duralım. Rust dilinde birçok smart pointer vardır. Box, RefCell, Rc, Arc gibi

- Pointer
    - Bellek üzerindeki bir veri içeriğini işaret eden adres bilgisini taşıyan değişken olarak düşünülebilir.
    - Farkında olmadan şu ana kadar kullandığımız bir pointer vardır (&) ve datayı referans etmekte kullanılır.
- Smart Pointer
    - Pointer adreslerine ek metadata bilgileri veya kabiliyetler ekler. Rust diline özel bir kavram değildir
      esasında C++ orijinlidir.
    - Referanslar veriyi işaret ederken Smart Pointer’ lar genellikle sahipliğini de alır. String ve Vec<T>
      türleri smart pointer olarak da geçerler, zira belli bir bellek adresindeki verinin sahipliğini alırlar ve onu
      manipüle etmemize izin verirler.
    - **Deref** ve **Drop** trait’lerini implemente eden struct türleri olarak tasarlanabilirler _(Yani kendi Smart
      Pointer modellerimizi tasarlayabiliriz)_

## Boxing

Bir veriyi Stack yerine Heap üzerinde konuşlandırmanın en basit hali Box enstrümanını kullanmaktır. Aşağıda bu kullanıma
ait basit bir fonksiyon yer almaktadır.

```rust
pub fn simple_boxing() {
    let value = 23; // Normalde stack' de saklanır
    let boxed_value = Box::new(value); // Şimdi heap'e alındı ama boxed_value hala stack'te zira adres göstermekte
    println!("Boxed value is {}", boxed_value);

    let identity = ("John Smith", 23, true); // tuple veriyi stack'ta saklar
    let boxed_identity = Box::new(identity); // Şimdi heap' te
    println!("Boxed identity is {:?}", boxed_identity);
}
```

value isimli değişken sayısal bir değerdir ve normal şartarda Stack'te tutulur. Box ile söz konusu değişken verisi
heap'e alınır ve stack'de onu işaret eden bir işaretçi bırakılır. Benzer bir kullanım şeklide Tuple veri türü içinde ele
alınmıştır. Box türünün kullanımı için sıkça vurgulanan senaryolardan birisi ağaç boğum modelleridir _(Tree Nodes)_
Aşağıdaki örnek kod parçasını göz önüne alalım.

```rust
fn main() {
    recursive_data_model_with_error()
}
enum Tree {
    Node(i32, Tree, Tree),
    Empty,
}

pub fn recursive_data_model_with_error() {
    let left_child = Tree::Node(1, Tree::Empty, Tree::Empty);
    let right_child = Tree::Node(3, Tree::Empty, Tree::Empty);
    let root = Tree::Node(2, left_child, right_child);
}
```

Bu kod parçası derlenmeyecektir ve şöyle bir hata mesajı verecektir.

```text
error[E0072]: recursive type `Tree` has infinite size
```

Buradakine benzer recursive veri modellerinden datanın ne kadar yer kaplayacağı derleme zamanında bilinemez. Senaryoda
enum türü kullanıldığı için de stack önceklikli bir yer ayırma durumu söz konusudur. Ne kadar boyut kaplanacağının
bilinmemesi taşma hatalarına sebebiyet verebilir. Bir düğüm kendisine referans verdikçe bu sonsuz boyutlamaya doğru
gidebilir. Dolayısıyla veriyi Heap üzerinde konuşlandırmak daha mantıklıdır. Benzer bir senaryoyu bu sefer aşağıdaki
gibi tasarlayarak devam edelim.

```rust
use std::fmt::{Display, Formatter};

enum Server {
    Node(String, Box<Server>, Box<Server>),
    Empty,
}
impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Server::Node(name, primary, backup) => {
                write!(
                    f,
                    "Server: {}\n  Primary: {}\n  Backup: {}",
                    name, primary, backup
                )
            }
            Server::Empty => write!(f, "None"),
        }
    }
}

pub fn recursive_sample() {
    let backup_server_blue = Server::Node(
        String::from("Backup Server Blue"),
        Box::new(Server::Empty),
        Box::new(Server::Empty),
    );

    let primary_server_green = Server::Node(
        String::from("Primary Server Green"),
        Box::new(Server::Empty),
        Box::new(backup_server_blue),
    );

    let root_server = Server::Node(
        String::from("Root Server"),
        Box::new(primary_server_green),
        Box::new(Server::Node(
            String::from("Backup Root"),
            Box::new(Server::Empty),
            Box::new(Server::Empty),
        )),
    );

    println!("{}", root_server);
}
```

Server isimli enum türünün Node alanı içerisinde Box edilen kendi türleri tutulmakta. Burada Box işlemi söz konusu
olduğu için bağlı liste Heap üzerinde konuşlandırılacak.

# Reference Counting

Bilinçli bir şekilde Heap üzerine alınan verilerde birden fazla sahipliğin söz konusu olduğu durumlarda referans
değerlerin sayımı tutulur. Eğer paylaşımlı thread'ler söz konusu değilse Rc türü ihtiyacı karşılar. Birden fazla thread
aynı veri alanı üzerinde çalışma gerektiği durumlarda ise tip güvenliğini gözeten Atomic Reference Counting yani Arc
kullanılır. Aşağıdaki kod parçasında en basit haliyle Rc türünden bir smart pointer kullanımı işlenmektedir.

```rust
use std::rc::Rc;

fn main() {
    hello_rc()
}

pub fn hello_rc() {
    let p1 = Rc::new(String::from("Some values on the heap"));
    let p2 = p1.clone();
    let p3 = p2.clone();

    println!("p1={:?}", p1);
    println!("p2={:?}", p2);
    println!("p3={:?}", p3);
}
```

Bu örnekte yer alan p1, p2 ve p3 değişkenleri aynı string veriyi içeren işaretçilerdir. Rc türüne olan ihtiyacı daha iyi
anlamak için aşağıdaki basit örneğe bakalım.

```rust
fn main() {
    run_rc_with_error()
}

#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
    friends: Vec<Player>,
}

impl Player {
    fn new(id: u32, name: &str) -> Self {
        Player {
            id,
            name: name.to_string(),
            friends: Vec::new(),
        }
    }

    fn add_friend(&mut self, friend: Player) {
        self.friends.push(friend);
    }

    fn print(&self) {
        println!("{}'s friends:", self.name);
        for friend in &self.friends {
            println!("  {} (ID: {})", friend.name, friend.id);
        }
    }
}

pub fn run_rc_with_error() {
    let mut steve = Player::new(1, "Stivi Vondır");
    let lord = Player::new(2, "Lord veyda");
    let anakin = Player::new(3, "Anakin");

    steve.add_friend(lord); // lord' un sahipliği add_friend sonrası steve' e taşındı
    steve.add_friend(anakin);

    steve.print();

    println!("Lord veyda's ID: {}", lord.id); // Value Moved Here
}
```

Bir oyuncunun arkadaşlarını da yine kendi türünden Vector olarak tutan Player isimli bir veri yapısı mevcut. add_friend
metodu ile bir oyuncuya başka Player örnekleri ekleyebiliyoruz. Player'ın sahip olduğu veri üzerinde değişiklik söz
konusu. Temsili run metoduna baktığımızda son satırdaki println! çağrısında value moved here hatası alırız. Bu son
derece doğaldır zira steve değişkeni üzerinden yapılan ilk add_friend çağrısı sırasında lord değişkeninin sahipliği de
taşınır. Dolayısıyla add_friend sonrası lord değişkenine tekrardan erişilemez. Bu tip bir senaryoyu yönetmek için Rc
smart pointer kullanılabilir. Ancak mutable olma zorunluluğuna dikkat etmek gerekir. Bunu daha iyi anlamak için örneği
aşağıdaki haliyle değiştirelim.

```rust
#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
    friends: Vec<Rc<Player>>,
}

impl Player {
    fn new(id: u32, name: &str) -> Rc<Self> {
        Rc::new(Player {
            id,
            name: name.to_string(),
            friends: Vec::new(),
        })
    }

    fn add_friend(self: &Rc<Self>, friend: Rc<Player>) {
        self.friends.push(friend);
    }

    fn print(&self) {
        println!("{}'s friends:", self.name);
        for friend in self.friends.iter() {
            println!("  {} (ID: {})", friend.name, friend.id);
        }
    }
}

pub fn run_rc_with_error_2() {
    let steve = Player::new(1, "Stivi Vondır");
    let lord = Player::new(2, "Lord veyda");
    let anakin = Player::new(3, "Anakin");

    steve.add_friend(Rc::clone(&lord));
    steve.add_friend(Rc::clone(&anakin));

    steve.print();

    println!("Lord Veyda's ID: {}", lord.id);
}
```

Bu sefer add_friend metodu içerisindeki self.friends.push metodunda bir hata alınır.

```text
cannot borrow data in an `Rc` as mutable [E0596]
cannot borrow as mutable
Help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<Player>`
```

Player veri yapısı kendi içerisinden kendi türünden bir Vector kullanmaktadır.İlk hata sebebiyle Vec'ün Rc<Player>
şeklinde kullanılması tercih edilebilir. Ancak bu özellikle add_friends metodunda vektör içeriğine mutable erişmeyi
gerektirir. Bu nedenle vektöre referansının da mutable olarak ele alınabilmesi gerekir. Normalde bir veriye erişen
birden fazla sahip varken mutable kullanım derleme hatasına yol açabilir. **RefCell** smart pointer kullanımı ile bunu
çalışma zamanına taşırız. Yani ownership kontrolünü runtime tarafında işletilmesini sağlarız. Dolayısıyla örnek kodları
aşağıdaki şekilde değiştirerek ilerleyebiliriz.

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    run_rc()
}

#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
    friends: RefCell<Vec<Rc<Player>>>,
}

impl Player {
    fn new(id: u32, name: &str) -> Rc<Self> {
        Rc::new(Player {
            id,
            name: name.to_string(),
            friends: RefCell::new(Vec::new()),
        })
    }

    fn add_friend(self: &Rc<Self>, friend: Rc<Player>) {
        self.friends.borrow_mut().push(friend);
    }

    fn print(&self) {
        println!("{}'s friends:", self.name);
        for friend in self.friends.borrow().iter() {
            println!("  {} (ID: {})", friend.name, friend.id);
        }
    }
}

pub fn run_rc() {
    let steve = Player::new(1, "Stivi Vondır");
    let lord = Player::new(2, "Lord veyda");
    let anakin = Player::new(3, "Anakin");

    steve.add_friend(Rc::clone(&lord));
    steve.add_friend(Rc::clone(&anakin));

    steve.print();

    println!("Lord Veyda's ID: {}", lord.id);
}
```

İlk dikkat edilmesi gereken nokta Player veri yapısındaki friends alanının türüdür. Player nesneleri için bir referans
sayacı kullanılırken değiştirilebilir olmasını sağlama işi RefCell ile çalışma zamanına bırakılmıştır. new metodu
içerisinde RefCell nesnesnin nasıl kullanıldığına da dikkat edelim. Ayrıca friends vektörünü üzerinde değişiklik yapmak
üzere kullanacaksak aynen add_friend metodunda olduğu gibi borrow_mut fonksiyonu ile mutable olarak ödünç alınmasını
sağlamalıyız. Eğer sadee okuma amaçlı kullanacaksak bu durumda da borrow metodunu kullanmalıyız.

Bu senaryoya göre farklı kullanım şekilleri de söz konusu olabilir.

- Sadece bir vektör üzerinde çalışılacaksa RefCell<Vec<Player>> kullanımı yeterlidir.
- Vektörün paylaşımı söz konusu ise Rc<RefCell<Vec<Player>>> daha uygun bir çözüm olabilir.
- Hem vektörü hem de içindeki elemanların paylaşışması gerekiyorsa Rc<Vec<RefCell<Player>>>
  daha iyi bir çözüm olabilir.

Şunu da unutmamamak gerekir hem Rc hem de RefCell kullanımının çalışma zamanı maliyetleri daha yüksektir _(Zira referans
sayımı ve mutasyon kontrolleri yapılmaktadır)_

Buraya kadar gördüğümüz Smart Pointer türlerini aşağıdaki grafikle özetleyebiliriz.

![Smart Pointers.png](smrt_ptrs.png)

## Atomic Reference Counting

// Thread'lerin işlendiği bölümde ele alınacaktır

## Hangisi ne zaman?

**Box** ve **RefCell** birden fazla sahipliği tek bir veri üzerinde sağlarken, Rc aynı veri üzerinden birden fazla
sahiplik sunar. Box immutable veya mutable ödünç alma _(borrowing)_ için derleme zamanında kontroller sağlar. Rc sadece
immutable borrowing için derleme zamanında kontrol sağlar. RefCell immutable veya mutable ödünç alma için runtime'da
kontrol sağlar.