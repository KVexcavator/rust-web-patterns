// Конфигурирование функций с помошью трейтов
// Пример демонстрирет, как добавить ещё один доступ к БД для получения имени юзера

// орпедление trait
#[derive(Debug)]
pub struct User {
  name: String,
  age: u32,
}

trait GetUsers {
  fn get_users() -> Vec<User>;
  fn get_user_by_name(name: &str) -> Option<User> {
    let users = Self::get_users();
    for user in users {
      if user.name == name {
        return Some(user);
      }
    }
    None
  }
}

// trait for DB handle
pub struct PostgresDB;

impl GetUsers for PostgresDB {
  fn get_users() -> Vec<User> {
    vec![
      User {
        name: "John".to_string(),
        age: 30,
      },
      User {
        name: "Jana".to_string(),
        age: 25,
      },
    ]
  }
}

// контракт для обрабтки ввода вывода операций DB
#[derive(Debug)]
pub struct GetUserContract {
  pub name: String,
  pub user: Option<User>,
}

fn handle_get_user_by_name<T, GetUsers>(contract: GetUserContract) -> GetUserContract {
  let user = T::get_user_by_name(&contract.name);
  GetUserContract {
    name: contract.name,
    user: user,
  }
}

// вызов макроса с трайтом
// трайт определен в macros_metta

#[derive(Debug)]
pub enum ContractHandler {
    ContractOne(ContractOne),
    ContractTwo(ContractTwo),
    GetUserContract(GetUserContract)
}

register_contract_routes!(
    ContractHandler,
    handle_contract,
    ContractOne => handle_contract_one,
    ContractTwo => handle_contract_two,
    GetUserContract => handle_get_user_by_name::<PostgresDB>
);

fn main() {
    // see macros_metta
        let get_user_contract = GetUserContract {
            name: "John".to_string(),
            user: None
        };

        let outcome = handle_contract(
            ContractHandler::GetUserContract(
            get_user_contract
        )
    );
    println!("{:?}", outcome);
}