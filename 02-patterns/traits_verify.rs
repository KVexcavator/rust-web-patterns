// обработка разных типов с помощью трейтов

struct AdminUser {
  username: String,
  password: String,
}

struct User {
  username: String,
  password: String,
}

trait CanEdit {
  fn edit(&self) {
    println!("Admin is editing");
  }
}

trait CanCreate {
  fn create(&self) {
    println!("Admin is creating");
  }
}

trait CanDelete {
  fn delete(&self) {
    println!("Admin is deleting");
  }
}

impl CanDelete for AdminUser {}
impl CanCreate for AdminUser {}
impl CanEdit for AdminUser {}

impl CanEdit for User {
  fn edit(&self) {
    println!("A standart user {} is editting", self.username);
  }
}

// так функции исползуются
fn create<T: CanCreate>(user: &T) {
  user.create();
}
fn edit<T: CanEdit>(user: &T) {
  user.edit();
}
fn delete<T: CanDelete>(user: &T) {
  user.delete();
}

fn main() {
  let admin = AdminUser{
    username: "Boss".to_string(),
    password: "password".to_string(),
  };
  let user = User{
    username: "Pupkin".to_string(),
    password: "password".to_string(),
  };

  create(&admin);
  edit(&admin);
  edit(&user);
  delete(&admin);
}

// Admin is creating
// Admin is editing
// A standart user Pupkin is editting
// Admin is deleting