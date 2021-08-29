use std::fmt::Display;

pub struct Index<'a>(&'a str);

impl<'a> From<&'a str> for Index<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}

struct Capitalize<'a>(&'a str);

impl<'a> Display for Capitalize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (index, fst) = match self.0.chars().next() {
            Some(c) => (c.len_utf8(), c),
            None => return Ok(()),
        };

        let (_, tail) = self.0.split_at(index);

        write!(f, "{}{}", fst.to_uppercase(), tail)
    }
}

impl<'a> Display for Index<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        let s = match self.0 {
            "1º secretário do IAGP" => vec!["IAGP"],
            "Ribeiro da Silva" => vec!["Silva", "Ribeiro da"],
            "1º Secretário da Biblioteca Pública Pelotense" => vec!["Biblioteca Pública Pelotense"],
            "1º Tenente João Augusto Cezar da Silva" => vec!["Silva", "João Augusto Cezar da"],
            "3º Congresso Scientifico Latino-Americano"
            | "Comissão Directora do Congresso Scientifico Latino-Americano"
            | "Congresso Scientifico Latino-Americano" => {
                vec!["Congresso Científico Latino-Americano"]
            }
            "A Directoria Geral da Instrucção Publica de São Paulo" => {
                vec!["Instrucção Publica de São Paulo"]
            }
            "A Exma. Sra. D. Ignez Maranhão" => vec!["Maranhão", "D. Ignez"],
            "A redação da Revista"
            | "A redação da revista"
            | "A redação do periódico"
            | "Adquirido pelo IHGRN"
            | "Da redação da Revista"
            | "Da redação da revista"
            | "Da redação do jornal" => return Ok(()),
            "A. J. Raposo da Camara" => vec!["Camara", "A. J. Raposo da"],
            "A. Tavares de Lyra"
            | "Sócio A. Tavares de Lyra" 
            | "Augusto Tavares de Lyra"
            | "Sócio Augusto Lyra" 
            | "Sócio Dr. Augusto Tavares de Lyra"
            | "Sócio Tavares de Lyra"
            | "Tavares de Lyra"
            | "Tavares de Lyra e Governador do Estado"
            | "Tavares Lyra"
            | "Consorcio Tavares Lyra" => vec!["Lyra", "Augusto Tavares de"],
            "Abel Furtado" => vec!["Furtado", "Abel"],
            "Adquirido pelo Instituto através do sócio Henrique Castriciano, no valor de trinta e cinco mil réis"
            | "Sócio Henrique Castriciano" => vec!["Castriciano", "Henrique"],
            "Affonso Costa" => vec!["Costa", "Affonso"],
            "Alberto Maranhão"
            | "Sócio Alberto Maranhão"
            | "Consocio Alberto Maranhão"
            | "Consorcio Alberto Maranhão"
            | "Consócio benemerito Alberto Maranhão" => vec!["Maranhão", "Alberto"],
            "Alcides Câmara" => vec!["Câmara", "Alcides"],
            "Alfredo de Carvalho" | "Consocio Alfredo de Carvalho" => vec!["Carvalho", "Alfredo de"],
            "Alfredo Herculano Barbalho" => vec!["Barbalho", "Alfredo Herculano"],
            "Antonio Alexandre Borges Reis" => vec!["Reis", "Antonio Alexandre Borges"],
            "Antonio Alves Camara" => vec!["Camara", "Antonio Alves"],
            "Antonio Fereira de Souza Pitanga"
            | "Desembargador A. F. de Souza Pitanga"
            | "Desembargador Antonio Ferreira de Souza Pitanga" => vec!["Pitanga", "Antonio Fereira de Souza"],
            "Antonio José de Mello e Souza" => vec!["Mello e Souza", "Antonio José de"],
            "Antonio Pereira de Figueiredo" => vec!["Pereira", "Antonio de Figueiredo"],
            "Antonio Virgilio de Miranda" => vec!["Miranda", "Antonio Virgilio de"],
            "Antônio Soares" => vec!["Soares", "Antônio"],
            "Archivo Maçônico" => vec!["Arquivo Maçônico"],
            "Archivo Municipal de Curityba" => vec!["Arquivo Municipal de Curitiba"],
            "Arquivo Municipal de Curityba" => vec!["Arquivo Municipal de Curitiba"],
            "Arquivo público nacional"
            | "Archivo Público Nacional"
            | "Director do do Archivo Publico Dr. Vicente Vianna"
            | "Director do Archivo Publico Nacional" => vec!["Arquivo Público Nacional"],
            "Associação Commercial do Maranhão" => vec!["Associação Comercial do Maranhão"],
            "Augusto Meira" => vec!["Meira", "Augusto"],
            "Augusto Tavares de Lyra e Alberto Maranhão" => {
                Display::fmt(&Index::from("Augusto Tavares de Lyra"), f)?;
                Display::fmt(&Index::from("Alberto Maranhão"), f)?;
                return Ok(())
            }
            "Barão de Studart" => vec!["Studart", "Barão de"],
            "Barão Feachtuelebeu" => vec!["Feachtuelebeu", "Barão"],
            "Belisario Pernambuco" => vec!["Pernambuco", "Belisario"],
            "Bibliotheca da Faculdade de Direito do Recife" => vec!["Biblioteca da Faculdade de Direito do Recife"],
            "Bibliotheca da Faculdade de Medicina da Bahia" => vec!["Biblioteca da Faculdade de Medicina da Bahia"],
            "Bibliotheca do Centro Academico de Agosto" => vec!["Biblioteca do Centro Acadêmico de Agosto"],
            "Bibliotheca do Centro Acadêmico de Agosto" => vec!["Biblioteca do Centro Acadêmico de Agosto"],
            "Bibliotheca e Archico Publico do Pará"
            | "Bibliotheca e Archivo do Pará"
            | "Bibliotheca e Archivo Publico do Pará"
            | "Director da Bibliotheca e Archivo Publico de Pará"
            => vec!["Pará", "Biblioteca e Arquivo Público do Pará"],
            "Bibliotheca Nacional do Rio de Janeiro"
            | "Director da Bibliotheca Nacional" => vec!["Biblioteca Nacional"],
            "Bibliotheca Publica de Pernambuco" => vec!["Biblioteca Pública de Pernambuco"],
            "Bibliotheca Publica de Sergipe" => vec!["Biblioteca Pública de Sergipe"],
            "Bibliotheca Publica Pelotense" => vec!["Biblioteca Pública Pelotense"],
            // "Bibliotheca da Faculdade de Direito do Recife" => "Biblioteca da Faculdade de Direito do Recife",
            "Bilac Guimarães Passos e Bandeira Junior" => {
                Display::fmt(&Index::from("Bilac Guimarães Passos"), f)?;
                Display::fmt(&Index::from("Bandeira Junior"), f)?;
                return Ok(())
            },
            "Bispo do Piauhy" => vec!["Bispo do Piauí"],
            "Camara episcopal" => vec!["Camara Episcopal"],
            "Camillo Castello Branco" => vec!["Castello Branco", "Camillo"],
            "Camillo Catello Branco" => vec!["Castello Branco", "Camillo"],
            "Capitão Luis Eugenio Ferreira Veiga" => vec!["Veiga", "Luis Eugenio Ferreira"],
            "Carlos Weber" => vec!["Weber", "Carlos"],
            "Centro Academico 11 de Agosto"
            | "Centro Acadêmico 11 de Agosto" => vec!["Centro Acadêmico 11 de Agosto"],
            | "Centro Acadêmico 11 de Agosto, Faculdade de Direito de S. Paulo" => {
                Display::fmt(&Index::from("Centro Acadêmico 11 de Agosto"), f)?;
                Display::fmt(&Index::from("Faculdade de Direito de S. Paulo"), f)?;
                return Ok(())
            },
            "Cidade de Natal" => vec!["Natal", "Cidade de"],
            "Cidadão Theodorico de Souza Caldas" => vec!["Caldas", "Theodorico de Souza"],
            "Cleodon Aranha" => vec!["Aranha", "Cleodon"],
            "Club de Engenharia" => vec!["Club de Engenharia"],
            "Club de Engenharia do Rio de Janeiro" => vec!["Club de Engenharia", "do Rio de Janeiro"],
            "Club Litterario 13 de Maio" => vec!["Club Litterario 13 de Maio"],
            "Club União e Perseverança de Belém, Pará" => vec!["Club União e Perseverança de Belém, Pará"],
            "Coelho Rodrigues" => vec!["Coelho Rodrigues"],
            "Comissão de açudes e irrigação" => vec!["Comissão de Açudes e irrigação"],
            "Comissão Geographica e Geologica de São Paulo"
            | "Comissão geographica Geologico do Estado de São Paulo"
            | "Commissão geographica e geológica do Estado de S. Paulo"
            | "Commissão geographica e geológica do Estado de São Paulo" => vec!["Comissão Geográfica e Geológica do Estado de São Paulo"],
            "Commissão Central" => vec!["Commissão Central"],
            "Commissão Geologica de São Paulo" => vec!["Comissão Geológica de São Paulo"],
            "Commissão de pesquizas de documentos" => vec!["Commissão de Pesquizas de Documentos"],
            "Congresso Litterario de Natal" => vec!["Natal", "Congresso Literário de"],
            "Congresso Litterario Tibiriçá de Lemos"
            | "Congresso Litterario Tibiriçá de Lemos”"
            | "Congresso Litterario “Tibiriçá de Lemos”"
            | "Congresso Tibiriçá de Lemos" => vec!["Congresso Litterario Tibiriçá de Lemos"],
            "Conselheiro Ruy Barbosa" => vec!["Barbosa", "Ruy"],
            "Consocio Senador Ferreira Chaves" => vec!["Ferreira Chaves"],
            "Consócio Francisco Câmara" => vec!["Câmara", "Francisco"],
            "Coronel Gregorio Thaumaturgo de Azevedo" => vec!["Azevedo", "Gregorio Thaumaturgo de"],
            "Coronel J. J. Valentim de Almeida" => vec!["Almeida", "J. J. Valentim de"],
            "Cultura Acadêmica" => vec!["Cultura Acadêmica"],
            "Cópia de documento" => vec!["Cópia de Documento"],
            "D. Joaquim de Almeida" => vec!["Almeida", "Joaquim de"],
            "Delegacia Fiscal do Thesouro Federal" => vec!["Delegacia Fiscal", "do Thesouro Federal"],
            "Delegacia Fiscal do Thesouro Nacional de São Paulo" => vec!["Delegacia Fiscal", "do Thesouro Nacional de São Paulo"],
            "Dionysio Filgueira" => vec!["Filgueira", "Dionysio"],
            


            "Sócio Arthur Lisboa" => vec!["Lisboa", "Arthur"],
            "Sócio Carvalho e Souza" | "Carvalho e Souza" => vec!["Carvalho e Souza"],
            "Sócio Correspondente Dr. Sebastião de Vasconcellos Galvão" | "Sócio correspondente Sebastião de Vasconcellos Galvão" => vec!["Galvão", "Sebastião de Vasconcellos"],
            "Sócio correspondente Irineu Ferreira Pinto"
            | "Sócio Irineu Ferreira Pinto" => vec!["Pinto", "Irineu Ferreira"],
            "Sócio Correspondente João de Lyra Tavares"
            | "Sócio correspondente João de Lyra Tavares"
            | "Cel. João de Lyra Tavares"
            | "Coronel João de Lyra Tavares" => vec!["Tavares", "João de Lyra"],
            "Sócio Correspondente Manuel Praxedes"
            | "Correspondente Manuel Prexades" => vec!["Praxedes", "Manuel"],
            "Sócio do IAGP, Pereira da Costa" => vec!["Pereira da Costa"],
            "Sócio Dr. Eloy de Souza"
            | "Sócio Eloy de Souza"
            | "Eloy de Souza" => vec!["Souza", "Eloy de"],
            "Sócio dr. José Correia" => vec!["Correia", "José"],
            "Sócio Dr. Meira e Sá" | "Sócio Meira e Sá" => vec!["Meira e Sá"],
            "Desembargador Francisco S. Meira e Sá" => vec!["Meira e Sá", "Francisco S."],
            "Sócio Dr. Vicente de Lemos"
            | "Sócio Vicente de Lemos"
            | "Vicente Lemos"
            | "Vicente de Lemos" => vec!["Lemos", "Vicente de"],
            "Sócio Ezequiel Medeiros" => vec!["Medeiros", "Ezequiel"],
            "Sócio Honorário Dr. Manoel Pereira Reis" => vec!["Reis", "Manoel Pereira"],
            "Sócio Honório Carilho"
            | "Sócio Honório Carrilho" => vec!["Carrilho", "Honório"],
            "Sócio Joaquim Lourival"
            | "Sócio Lourival" => vec!["Lourival", "Joaquim"],
            "Sócio Joaquim Manuel" => vec!["Manuel", "Joaquim"],
            "Sócio Lins Caldas" => vec!["Caldas", "Lins"],
            "Sócio Luiz Fernandes" => vec!["Fernandes", "Luiz"],
            "Sócio Manuel Hemeterio" => vec!["Hemeterio", "Manuel"],
            "Sócio Monsenhor José Paulino de Andrada" => vec!["Andrada", "José Paulino de"],
            "Sócio Olympio Vital" | "Sócio Olímpio Vital" | "Olympio Vital" | "Olímpio Vital" => vec!["Vital", "Olympio"],
            "Sócio Pedro Soares" => vec!["Soares", "Pedro"],
            "Sócio Pedro Velho" => vec!["Velho", "Pedro"],
            "Sócio Pereira Simões" | "Consorcio Pereira Simões" => vec!["Pereira Simões"],
            "Sócio Pinto de Abreu" => vec!["Pinto de Abreu"],
            "Sócio Segundo Wanderley"
            | "Segundo Wanderley"
            | "Comissão encarregada de constituir um patrimônio para os orphãos do Dr. Manoel Segundo Wanderley" => vec!["Wanderley", "Manoel Segundo"],
            "Sócio Valle Miranda" => vec!["Valle Miranda"],
            "Sócio Vicente Ferrer"
            | "Vicente Ferrer de B. W. Araújo"
            | "Ferrer de Barros Wanderley Araujo" => vec!["Araújo", "Vicente Ferrer de Barros Wanderley"],
            "Sócios Augusto Tavares de Lyra e Vicente Lemos"
            | "Tavares de Lyra e Vicente de Lemos" => {
                Display::fmt(&Index::from("Augusto Tavares de Lyra"), f)?;
                Display::fmt(&Index::from("Vicente Lemos"), f)?;
                return Ok(())
            }
            "Telemaco Cicero Peneira e Silva" => vec!["Silva", "Telemaco Cicero Peneira e"],
            "Tenente Alipio Bandeira"
            | "Tenente dr. Alipio Bandeira" => vec!["Bandeira", "Alipio"],
            "Tenente Coronel Francisco Cascudo" => vec!["Cascudo", "Francisco"],
            "Tenente J. Vieira da Rosa" => vec!["Rosa", "J. Vieira da"],
            "Thomaz Pompeu Souza Brazil" => vec!["Brazil", "Thomaz Pompeu Souza"],
            "Tipografia Minerva" => vec!["Tipografia Minerva"],
            "University Press, da California" => vec!["University Press, California"],
            "Urbano Hermillo de Mello" => vec!["Mello", "Urbano Hermillo de"],
            "Vicente de Lemos Filho" => vec!["Lemos Filho", "Vicente de"],

            s => {
                f.write_str(r"\index{")?;

                let mut it = s.splitn(2, " ");

                it.next()
                    .map(|s| write!(f, "{}", Capitalize(s)))
                    .transpose()?;

                for s in it {
                    write!(f, "!{}", s)?;
                }

                return f.write_str("}")
            },
        };

        f.write_str(r"\index{")?;

        let mut it = s.into_iter();

        it.next()
            .map(|s| write!(f, "{}", Capitalize(s)))
            .transpose()?;

        for s in it {
            write!(f, "!{}", s)?;
        }

        f.write_str("}")
    }
}
