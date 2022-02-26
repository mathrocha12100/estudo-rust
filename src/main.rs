mod aulas;
mod file;

enum Aulas {
    AulaConst,
    AulaEnum,
    AulaFactorial,
    AulaInputOutputData,
    AulaReferences,
    AulaRsFor,
    AulaRsWhile,
    AulaShadowing,
    AulaStructs,
    AulaTuple,
    AulaString,
    ArraysRs,
    Keyword,
    VectorRs,
    HashMapo,
    File,
    Traits,
    PatternMatching
}

fn main() {
    let aula: Aulas = Aulas::Traits;

    match aula {
        Aulas::AulaStructs => aulas::structs_rs::structs_rs(),
        Aulas::AulaConst => aulas::const_rs::const_rs(),
        Aulas::AulaEnum => aulas::enum_rs::enum_rs(),
        Aulas::AulaInputOutputData => aulas::input_output_dados::exemplo_entrada_saida_dados(),
        Aulas::AulaRsFor => aulas::rs_for::rs_for(),
        Aulas::AulaRsWhile => aulas::rs_while::rs_while(),
        Aulas::AulaShadowing => aulas::shadowing::shadowing_rs(),
        Aulas::AulaTuple => aulas::tuple_rs::tuple_rs(),
        Aulas::AulaReferences => aulas::references_rs::references_rs(),
        Aulas::AulaFactorial => aulas::factorial_rs::factorial_rs(),
        Aulas::AulaString => aulas::string_rs::string_rs(),
        Aulas::ArraysRs => aulas::arrays_rs::arrays_rs(),
        Aulas::Keyword => aulas::keyword::keyword(),
        Aulas::VectorRs => aulas::vector_rs::vector_rs(),
        Aulas::VectorRs => aulas::vector_rs::vector_rs(),
        Aulas::HashMapo => aulas::hash_map::hash_map(),
        Aulas::File => file::file_rs::file_rs(),
        Aulas::Traits => aulas::traits::traits(),
        Aulas::PatternMatching => aulas::pattern_matching::pattern_matching()
    }
}   
