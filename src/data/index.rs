use std::{fmt::Display, vec};

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
            "1º secretário do IAGP"
            | "Instituto Archeologico e Geographico Pernambucano"
            | "Instituto Arqueológico e Geográfico de Pernambuco" => vec!["Instituto Arqueológico e Geográfico Pernambucano"],
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
            | "Da redação do jornal"
            | "Governador do Estado"
            | "Governo do Estado"
            | "IHGRN comprou"
            | "Journal universel hebdomadaire" => return Ok(()),
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
            | "Sócio Henrique Castriciano"
            | "Henrique Castriciano" => vec!["Castriciano", "Henrique"],
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
            | "Director do Archivo Publico Nacional"
            | "Directoria do Archivo Publico Nacional"
            | "Diretor do Arquivo Público Nacional" => vec!["Arquivo Público Nacional"],
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
            "Bibliotheca da Faculdade de Direito do Recife"
            | "Faculdade de Direito de Recife" => vec!["Biblioteca da Faculdade de Direito do Recife"],
            "Bibliotheca da Faculdade de Medicina da Bahia" => vec!["Biblioteca da Faculdade de Medicina da Bahia"],
            "Bibliotheca do Centro Academico de Agosto" => vec!["Biblioteca do Centro Acadêmico de Agosto"],
            "Bibliotheca do Centro Acadêmico de Agosto" => vec!["Biblioteca do Centro Acadêmico de Agosto"],
            "Bibliotheca e Archico Publico do Pará"
            | "Bibliotheca e Archivo do Pará"
            | "Bibliotheca e Archivo Publico do Pará"
            | "Director da Bibliotheca e Archivo Publico de Pará"
            | "Diretoria do Arquivo Público do Pará"
            => vec!["Pará", "Biblioteca e Arquivo Público do Pará"],
            "Bibliotheca Nacional do Rio de Janeiro"
            | "Director da Bibliotheca Nacional"
            | "Diretor da Biblioteca Nacional"
            | "Biblioteca Nacional" => vec!["Biblioteca Nacional"],
            "Bibliotheca Publica de Pernambuco" => vec!["Biblioteca Pública de Pernambuco"],
            "Bibliotheca Publica de Sergipe" => vec!["Biblioteca Pública de Sergipe"],
            "Bibliotheca Publica Pelotense" => vec!["Biblioteca Pública Pelotense"],
            "Bilac Guimarães Passos e Bandeira Junior" => {
                Display::fmt(&Index::from("Bilac Guimarães Passos"), f)?;
                Display::fmt(&Index::from("Bandeira Junior"), f)?;
                return Ok(())
            },
            "Bispo do Piauhy" => vec!["Bispo do Piauí"],
            "Camara episcopal" => vec!["Camara Episcopal"],
            "Camillo Castello Branco"
            | "Camillo Catello Branco" => vec!["Castello Branco", "Camillo"],
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
            "Consocio Senador Ferreira Chaves"
            | "Ferreira Chaves" => vec!["Ferreira Chaves"],
            "Consócio Francisco Câmara"
            | "Francisco Câmara" => vec!["Câmara", "Francisco"],
            "Coronel Gregorio Thaumaturgo de Azevedo" => vec!["Azevedo", "Gregorio Thaumaturgo de"],
            "Coronel J. J. Valentim de Almeida" => vec!["Almeida", "J. J. Valentim de"],
            "Cultura Acadêmica" => vec!["Cultura Acadêmica"],
            "Cópia de documento" => vec!["Cópia de Documento"],
            "D. Joaquim de Almeida" => vec!["Almeida", "Joaquim de"],
            "Delegacia Fiscal do Thesouro Federal" => vec!["Delegacia Fiscal", "do Thesouro Federal"],
            "Delegacia Fiscal do Thesouro Nacional de São Paulo" => vec!["Delegacia Fiscal", "do Thesouro Nacional de São Paulo"],
            "Dionysio Filgueira" => vec!["Filgueira", "Dionysio"],
            "Directoria da Agricultura, Commercio e Obras Publicas de S. Paulo" => vec!["Directoria da Agricultura, Comércio e Obras Públicas de S. Paulo"],
            "Directoria da Instrucção Publica do Uruguay" => vec!["Uruguay", "Diretoria da Instrução Pública do"],
            "Directoria do Archivo Publico Mineiro" => vec!["Arquivo Público Mineiro"],
            "Directoria do Club Carlos Gomes" => vec!["Club Carlos Gomes"],
            "Directoria do Museu Goeldi"
            | "Directoria do Museu Goeldi do Pará"
            | "Diretor do respectivo Museu, Dr. E. A. Goeldi"
            | "Museu Goeldi" => vec!["Museu Goeldi"],
            "Directoria do Serviço Sanitario de Belem" => vec!["Serviço Sanitário", "de Belém"],
            "Directoria do Serviço Sanitario do Pará"
            | "Directoria do serviço sanitario do Pará"
            | "Directoria do Serviço Sanitário do Pará" => vec!["Serviço Sanitário", "do Pará"],
            "Directoria Instrucção Publica de São Paulo" => vec!["São Paulo", "Diretoria Geral da Instrução Pública de"],
            "Directoria Geral de Estatistica"
            | "Directoria geral de Estatistica"
            | "Directoria Geral de Estatística" => vec!["Diretoria Geral de Estatística"],
            "Diretoria Geral de Estatística do Rio de Janeiro" => vec!["Diretoria Geral de Estatística", "do Rio de Janeiro"],
            "Diretoria da Indústria e Comércio" => vec!["Diretoria da Indústria e Comércio"],
            "Diretoria da Previdente Natalense" => vec!["Diretoria da Previdente Natalense"],
            "Diretoria do Grêmio Rio-grandense do Norte"
            | "Grêmio Rio Grandense do Norte" => vec!["Grêmio Rio-Grandense do Norte"],
            "Diretoria Geral dos Correios" => vec!["Correios"],
            "Diário oficial" => vec!["Diário Oficial"],
            "Do cidadão Fortunato Aranha"
            | "Fortunato Aranha" => vec!["Aranha", "Fortunato"],
            "Documento escolar" => vec!["Documento", "Escolar"],
            "Documento religioso" => vec!["Documento", "Religioso"],
            "Domingos Barros" => vec!["Barros", "Domingos"],
            "Dr. Felisbello Freire"
            | "Felisbelo Freire" => vec!["Freire", "Felisbello"],
            "Dr. Candido Duarte" => vec!["Duarte", "Candido"],
            "Dr. Costa Filho"
            | "Dr. Costa Filho," => vec!["Costa Filho"],
            "Dr. Ernesto Maranhão" => vec!["Maranhão", "Ernesto"],
            "Dr. Galdino Lima" => vec!["Lima", "Galdino"],
            "Dr. Goeldi" => vec!["Goeldi", "Émil August"],
            "Museu Paulista" => vec!["Museu Paulista"],
            "Dr. H. von Ihering, diretor do Museu Paulista." => {
                Display::fmt(&Index::from("Museu Paulista"), f)?;
                Display::fmt(&Index::from("H. von Jhering"), f)?;
                return Ok(())
            },
            "Dr. H. von Jhering" => vec!["Von Ihering", "Hermann"],
            "Dr. Henrique Guedes de Mello" => vec!["Mello", "Henrique Guedes de"],
            "Dr. J. C. Branner" => vec!["Branner", "J. C."],
            "Dr. J. C. Carneiro Monteiro" => vec!["Monteiro", "J. C. Carneiro"],
            "Dr. J. Castello Branco" => vec!["Castello Branco", "J."],
            "Dr. Joaquim Carlos Travassos" => vec!["Travassos", "Joaquim Carlos"],
            "Dr. José Augusto B. de Medeiros"
            | "Dr. José Augusto Bezerra de Medeiros"
            | "José Augusto B. de Medeiros"
            | "José Augusto Bezerra de Medeiros" => vec!["Medeiros", "José Augusto Bezerra de"],
            "Dr. José Zeferino da Cunha" => vec!["Cunha", "José Zeferino da"],
            "Dr. João Gualberto Machado Tinôco" => vec!["Tinôco", "João Gualberto Machado"],
            "Dr. João N. de Moura Soares"
            | "Dr. João Nepomuceno de Moura Soares"
            | "João Nepomuceno de Moura Soares" => vec!["Soares", "João Nepomuceno de Moura"],
            "Dr. Juvenal Antunes de Oliveira" => vec!["Oliveira", "Juvenal Antunes de"],
            "Dr. Manoel B. P. Diegues Junior" => vec!["Diegues Junior", "Manoel B. P."],
            "Dr. Mario Lyra" => vec!["Lyra", "Mario"],
            "Dr. Piquet Carneiro" => vec!["Piquet Carneiro"],
            "Dr. Raymundo Pereira da Silva" => vec!["Silva", "Raymundo Pereira da"],
            "Inspectoria de Obras Contra as Seccas" => vec!["Inspetoria de Obras contra as Secas"],
            "Dr. R. Pereira da Silva, da Inspectoria de Obras contra a secca" => {
                Display::fmt(&Index::from("Dr. Raymundo Pereira da Silva"), f)?;
                Display::fmt(&Index::from("Inspectoria de Obras Contra as Seccas"), f)?;
                return Ok(())
            }
            "Dr. Sebastião Paraná" => vec!["Paraná", "Sebastião"],
            "Dr.José Augusto Bezerra Guerra" => vec!["Guerra", "José Augusto Bezerra"],
            "Dr.João Baptista de Farias e Souza" => vec!["Farias e Souza", "João Baptista de"],
            "Empreza Editora de São Paulo" => vec!["Empresa Editora de São Paulo"],
            "Emygdio Moraes de Maranhão" => vec!["Maranhão", "Emygdio Moraes de"],
            "Escola Universitaria Livre de Manaus" => vec!["Escola Universitaria Livre de Manaus"],
            "Exma. Senhorita Carlota Lemos" => vec!["Lemos", "Carlota"],
            "Exma. Sra. D. Ignez Maranhão"
            | "Exma. sra. D. Ignez Maranhão," => vec!["Maranhão", "Ignez"],
            "Eça de Ôueiroz" => vec!["Queiroz", "Eça de"],
            "Ramalho Ortigão" => vec!["Ortigão", "Ramalho"],
            "Eça de Ôueiroz e Ramalho Ortigão" => {
                Display::fmt(&Index::from("Eça de Ôueiroz"), f)?;
                Display::fmt(&Index::from("Ramalho Ortigão"), f)?;
                return Ok(())
            }
            "F . Lino D’Assampção" => vec!["D’Assampção", "F. Lino"],
            "Federação Espirita Brazileira" => vec!["Federação Espirita Brazileira"],
            "Fortunato Aranho, em nome da Federação espirita Brazileira" => {
                Display::fmt(&Index::from("Fortunato Aranho"), f)?;
                Display::fmt(&Index::from("Federação Espirita Brazileira"), f)?;
                return Ok(())
            }
            "Francisco Palma" => vec!["Palma", "Francisco"],
            "Gabinete Portuguez de Leitura (Pernambuco)" => vec!["Pernambuco", "Gabinete Portuguez de Leitura de"],
            "Governador de Pernambuco, Antônio Gonçalves Ferreira" => vec!["Ferreira", "Antônio Gonçalves"],
            "Gremio Militar da Guarda Nacional" => vec!["Grêmio Militar da Guarda Nacional"],
            "Gremio Jaboatonense “Seis de Março”"
            | "Grêmio Jaboatanense “Seis de Março’" => vec!["Grêmio Jaboatonense “Seis de Março”"],
            "Gremio Literario da Parahyba , Alagoas" => vec!["Grêmio Literário da Paraíba, Alagoas"],
            "Gremio Litterario 3 de maio" => vec!["Grêmio Literário 3 de Maio"],
            "Gremio Litterario Augusto Severo"
            | "Gremmio Litterario Augusto Severo" => vec!["Grêmio Literário Augusto Severo"],
            "Gremio litterario Barbosa de Freitas" => vec!["Grêmio Literário Barbosa de Freitas"],
            "Gremio Litterario Mocidade Catholica" => vec!["Grêmio Literário Mocidade Católica"],
            "Gremio Litterario “Le Monde Marche”" => vec!["Gremio Litterario “Le Monde Marche”"],
            "Grêmio Tobias Barretto" => vec!["Grêmio Tobias Barretto"],
            "Guedes Alcoforado" => vec!["Guedes Alcoforado"],
            "H. Ramos" => vec!["Ramos", "H."],
            "Harb Theodor Walecchers" => vec!["Walecchers", "Harb Theodor"],
            "Heliodoro Barros" => vec!["Barros", "Heliodoro"],
            "Heraclides Camara" => vec!["Camara", "Heraclides"],
            "Herculano de Freitas" => vec!["Freitas", "Herculano de"],
            "Humberto de Campos" => vec!["Campos", "Humberto de"],
            "IHGB"
            | "Instituto Historico Brazileiro"
            | "Instituto Histórico e Geográfico Brasileiro" => vec!["Instituto Histórico Geográfico Brasileiro"],
            "IHGBA"
            | "Instituto Geographico e Historico da Bahia"
            | "Instituto Historico da Bahia"
            | "Instituto Histórico e Geográfico da Bahia" => vec!["Instituto Geográfico e Histórico da Bahia"],
            "Instituto de Humanidades do Ceará"
            | "Instituto do Ceará"
            | "Instituto Histórico do Ceará" => vec!["Instituto do Ceará"],
            "Instituto Histórico e Geographico da Parahyba" => vec!["Instituto Histórico e Geográfico da Paraíba"],
            "Instituto Histórico e Geográfico Alagoano" => vec!["Instituto Histórico e Geográfico Alagoano"],
            "Instituto Histórico e Geográfico de São Paulo" => vec!["Instituto Histórico e Geográfico de São Paulo"],
            "Instituto Litterario Mocidade Catholica de Mossoró" => vec!["Instituto Literário Mocidade Católica de Mossoró"],
            "Intendencia Municipal de Maceió" => vec!["Maceió", "Intendencia Municipal de"],
            "Irineu de Albuquerque" => vec!["Albuquerque", "Irineu de"],
            "Izabel Gondim" => vec!["Gondim", "Izabel"],
            "J. Augusto de Castro" => vec!["Castro", "J. Augusto de"],
            "J. Guerra-Rio" => vec!["Guerra-Rio", "J."],
            "J. P. Oliveira Martins" => vec!["Martins", "J. P. Oliveira"],
            "Joronymo Cabral Raposo da Camara" => vec!["Camara", "Joronymo Cabral Raposo da"],
            "Jeronymo Goeiros" => vec!["Goeiros", "Jeronymo"],
            "Joao Brigido dos Santos" => vec!["Santos", "João Brigido"],
            "Joaquim da Costa Nogueira" => vec!["Nogueira", "Joaquim da Costa"],
            "Joaquim Pinheiro" => vec!["Pinheiro", "Joaquim"],
            "Jose Villar" 
            | "José Villar" => vec!["Villar", "José"],
            "José Camara Lisboa" => vec!["Lisboa", "José Camara"],
            "José de Campos Novaes" => vec!["Novaes", "José de Campos"],
            "José Martins de Vasconcellos" => vec!["Vasconcellos", "José Martins de"],
            "José Salustiano Correia de Medeiros" => vec!["Medeiros", "José Salustiano Correia de"],
            "João F. da Frota e Vasconcellos [Bibliothecario da Faculdade de Direito do Recife]" => {
                Display::fmt(&Index::from("João F. da Frota e Vasconcellos"), f)?;
                Display::fmt(&Index::from("Bibliotheca da Faculdade de Direito do Recife"), f)?;
                return Ok(())
            }
            "João F. da Frota e Vasconcellos" => vec!["Vasconcellos", "João F. da Frota e"],
            "João Gualberto"
            | "João Gualberto Machado Tinoco" => vec!["Tinoco", "João Gualberto Machado"],
            "João Irineu Jóffely" => vec!["Jóffely", "João Irineu"],
            "João Nepomuceno Seabra de Mello" => vec!["Mello", "João Nepomuceno Seabra de"],
            "Julio Maria" => vec!["Julio Maria"],
            "Liga de Ensino" => vec!["Liga de Ensino"],
            "Loja maçônica 21 de Março" => vec!["Loja Maçônica", "21 de Março"],
            "Loja Maçônica Filhos da Fé" => vec!["Loja Maçônica", "Filhos da Fé"],
            "Luiz Antonio Fernandes Pimenta" => vec!["Pimenta", "Luiz Antonio Fernandes"],
            "M. E. Gomes de Carvalho" => vec!["Carvalho", "M. E. Gomes de"],
            "Major Codeceira"
            | "Major José Domingues Codeceira" => vec!["Codeceira", "José Domingues"],
            "Major Joaquim Soares" => vec!["Soares", "Joaquim"],
            "Major Pedro Dias de Campos" => vec!["Campos", "Pedro Dias de"],
            "Major Theodosio Paiva" => vec!["Paiva", "Theodosio"],
            "Manoel Barata" => vec!["Barata", "Manoel"],
            "Manoel Francisco da Trindade" => vec!["Trindade", "Manoel Francisco da"],
            "Manuel Cícero Peregrino da Silva" => vec!["Silva", "Manuel Cícero Peregrino da"],
            "Manuel Cícero Peregrino da Silva, diretor da Biblioteca Nacional" => {
                Display::fmt(&Index::from("Manuel Cícero Peregrino da Silva"), f)?;
                Display::fmt(&Index::from("Biblioteca Nacional"), f)?;
                return Ok(())
            }
            "Marechal Francisco de Paula Argollo" => vec!["Argollo", "Francisco de Paula"],
            "Maria Amélia Vaz de Carvalho" => vec!["Carvalho", "Maria Amélia Vaz de"],
            "Maria Suzana de Moura Salles" => vec!["Salles", "Maria Suzana de Moura"],
            "Maria Suzana de Moura Salles e o sócio Pinto de Abreu" => {
                Display::fmt(&Index::from("Maria Suzana de Moura Salles"), f)?;
                Display::fmt(&Index::from("Pinto de Abreu"), f)?;
                return Ok(())
            }
            "Mario Barros" => vec!["Barros", "Mario"],
            "Mario de Britto Barros" => vec!["Barros", "Mario de Britto"],
            "Militão Bivar" => vec!["Bivar Militão"],
            "Ministerio da Viação e Obras Publicas" => vec!["Ministerio da Viação e Obras Publicas"],
            "Ministro da Fazenda, dr. Leopoldo de Bulhões" => vec!["Bulhões", "Leopoldo de"],
            "Ministro da Industria, Viação e Oras Públicas, Lauro Severiano Muller" => vec!["Muller", "Lauro Severiano"],
            "Ministro da Justiça e Negocios Interiores, J. J. Seabra" => vec!["Seabra", "J. J."],
            "Ministro da marinha, vice-almirante Julio Cesar De Noronha" => vec!["Noronha", "Julio Cesar de"],
            "Monsenhor Sebastião Kneipp" => vec!["Kneipp", "Sebastião"],
            "Moyses Soares de Araujo"
            | "Moyses Soares de Araújo" => vec!["Araújo", "Moyses Soares de"],

            "Sócio Arthur Lisboa" => vec!["Lisboa", "Arthur"],
            "Sócio Carvalho e Souza" | "Carvalho e Souza" => vec!["Carvalho e Souza"],
            "Sócio Correspondente Dr. Sebastião de Vasconcellos Galvão"
            | "Sócio correspondente Sebastião de Vasconcellos Galvão"
            | "Dr.Sebastião de Vasconcellos Galvão" => vec!["Galvão", "Sebastião de Vasconcellos"],
            "Sócio correspondente Irineu Ferreira Pinto"
            | "Sócio Irineu Ferreira Pinto"
            | "Irineu Ferreira Pinto" => vec!["Pinto", "Irineu Ferreira"],
            "Sócio Correspondente João de Lyra Tavares"
            | "Sócio correspondente João de Lyra Tavares"
            | "Cel. João de Lyra Tavares"
            | "Coronel João de Lyra Tavares"
            | "João Lyra Tavares" => vec!["Tavares", "João de Lyra"],
            "Sócio Correspondente Manuel Praxedes"
            | "Correspondente Manuel Prexades" => vec!["Praxedes", "Manuel"],
            "Sócio do IAGP, Pereira da Costa" => vec!["Pereira da Costa"],
            "Sócio Dr. Eloy de Souza"
            | "Sócio Eloy de Souza"
            | "Eloy de Souza"
            | "Eloy de Sousa" => vec!["Souza", "Eloy de"],
            "Sócio dr. José Correia" => vec!["Correia", "José"],
            "Sócio Dr. Meira e Sá" | "Sócio Meira e Sá" | "Meira e Sá" => vec!["Meira e Sá"],
            "Desembargador Francisco S. Meira e Sá" => vec!["Meira e Sá", "Francisco S."],
            "Sócio Dr. Vicente de Lemos"
            | "Sócio Vicente de Lemos"
            | "Vicente Lemos"
            | "Vicente de Lemos"
            | "Dr. Lemos" => vec!["Lemos", "Vicente de"],
            "Sócio Ezequiel Medeiros" => vec!["Medeiros", "Ezequiel"],
            "Sócio Honorário Dr. Manoel Pereira Reis" => vec!["Reis", "Manoel Pereira"],
            "Sócio Honório Carilho"
            | "Sócio Honório Carrilho"
            | "Honorio Carrilho" => vec!["Carrilho", "Honório"],
            "Sócio Joaquim Lourival"
            | "Sócio Lourival" => vec!["Lourival", "Joaquim"],
            "Sócio Joaquim Manuel" => vec!["Manuel", "Joaquim"],
            "Sócio Lins Caldas" => vec!["Caldas", "Lins"],
            "Sócio Luiz Fernandes"
            | "Luiz Fernandes" => vec!["Fernandes", "Luiz"],
            "Sócio Manuel Hemeterio" => vec!["Hemeterio", "Manuel"],
            "Sócio Monsenhor José Paulino de Andrada"
            | "Monsenhor José Paulino de Andrada" => vec!["Andrada", "José Paulino de"],
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
