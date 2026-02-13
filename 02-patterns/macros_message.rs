// Mapping messages with macros
// Сообщения обрабатываются с помощью макросов, выполняя следующие шаги:
// 1. Определение контрактов и обработчика контрактов
#[derive(Debug)]
pub struct ContractOne {
  input_data: String,
  output_data: Option<Result<String, String>>,
}
#[derive(Debug)]
pub struct ContractTwo {
  input_data: String,
  output_data: Option<Result<String, String>>,
}
#[derive(Debug)]
pub enum ContractHandler {
  ContractOne(ContractOne),
  ContractTwo(ContractTwo),
}
// 2. Определение функций для обработки контрактов
fn handle_contract_one(mut contract: ContractOne) -> ContractOne {
    println!("{}", contract.input_data);
    contract.output_data = Some(Ok("Output Data".to_string()));
    contract
}

fn handle_contract_two(mut contract: ContractTwo) -> ContractTwo {
    println!("{}", contract.input_data);
    contract.output_data = Some(Ok("Output Data".to_string()));
    contract
}
// 3. Создание макроса для регистрации контрактов

// Обьяснение строки $( $contract:indent => $handler_
// fn:path,*):
// - $( и ),*: разделители указывают на шаблон повторения. $( начинает повторение, а ),* означает, что ни одно или несколько выражений не разделены запятыми. Это позволяет макросу принимать несколько пар contract => handler_fn
// - $contract:ident: $contract — это метапеременная. В макросах метапеременные — это заполнители, которые будут сопоставлены и заменены фактическим кодом или идентификаторами при развертывании макроса
// :ident указывает, что $contract должен соответствовать идентификатору. Идентификатор в Rust — это имя, используемое для переменных, функций, структур и т. д.
// - =>: Это литеральный токен, который должен отображаться точно так же, как и во входных данных макроса. Это разделяет контракт от пути к функции обработчика
// - $handler_fn:path: $handler_fn - метапеременная.:path указывает, что $handler_fn должен соответствовать пути
// В итоге, эта строка определяет макроправило, которое соответствует нулю или более парам контракт => handler_fn, позволяя нам легко сопоставлять контракты с соответствующими функциям

#[macro_export]
macro_rules! register_contract_routes {
  (
    $handler_enum:ident,
    $fn_name:ident,
    $( $contract:ident => $handler_fn:path ),
    *
  ) => {
    pub fn $fn_name(received_msg: $handler_enum) -> $handler_enum {
      match received_msg {
        // $(...)* is the loop
        $(
          $handler_enum::$contract(inner) => {
            let executed_contract = $handler_fn(
              inner
            );
            return $handler_enum::$contract(
              executed_contract
            )
          }
        )*
      }
    }
  };
}

// теперь можем определить
register_contract_routes!(
    ContractHandler,
    handle_contract,
    ContractOne => handle_contract_one,
    ContractTwo => handle_contract_two
);
// это развернется под капотом в 
// pub fn handle_contract(received_msg: ContractHandler)
// -> ContractHandler {
//     match received_msg {
//         ContractHandler::ContractOne(inner) => {
//             let executed_contract = handle_contract_one(
//                 inner
//             );
//             return ContractHandler::ContractOne(
//                 executed_contract
//             );
//         }
//         ContractHandler::ContractTwo(inner) => {
//             let executed_contract = handle_contract_two(
//                 inner
//             );
//             return ContractHandler::ContractTwo(
//                 executed_contract
//             );
//         }
//     }
// }

// 4. Использование макроса в программе
fn main() {
    let contract_one = ContractOne {
        input_data: "Contract One".to_string(),
        output_data: None
    };
    let outcome = handle_contract(
        ContractHandler::ContractOne(contract_one)
    );
    println!("{:?}", outcome);
}

// Contract One
// ContractOne(ContractOne { input_data: "Contract One", output_data: Some(Ok("Output Data")) })