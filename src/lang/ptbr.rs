lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Seu Desktop"),
        ("desk_tip", "Seu desktop pode ser acessado com este ID e senha."),
        ("Password", "Senha"),
        ("Ready", "Pronto"),
        ("Established", "Estabelecido"),
        ("connecting_status", "Conectando à rede do ClientDesk..."),
        ("Enable Service", "Habilitar Serviço"),
        ("Start Service", "Iniciar Serviço"),
        ("Service is running", "Serviço está em execução"),
        ("Service is not running", "Serviço não está em execução"),
        ("not_ready_status", "Não está pronto. Por favor verifique sua conexão"),
        ("Control Remote Desktop", "Controle o Desktop à distância"),
        ("Transfer File", "Transferir Arquivo"),
        ("Connect", "Conectar"),
        ("Recent Sessions", "Sessões recentes"),
        ("Address Book", "Lista de Endereços"),
        ("Confirmation", "Confirmação"),
        ("TCP Tunneling", "Tunelamento TCP"),
        ("Remove", "Remover"),
        ("Refresh random password", "Atualizar senha aleatória"),
        ("Set your own password", "Configure sua própria senha"),
        ("Enable Keyboard/Mouse", "Habilitar Teclado/Mouse"),
        ("Enable Clipboard", "Habilitar Área de Transferência"),
        ("Enable File Transfer", "Habilitar Transferência de Arquivos"),
        ("Enable TCP Tunneling", "Habilitar Tunelamento TCP"),
        ("IP Whitelisting", "Whitelisting de IP"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Stop service", "Parar serviço"),
        ("Change ID", "Alterar ID"),
        ("Website", "Website"),
        ("About", "Sobre"),
        ("Mute", "Emudecer"),
        ("Audio Input", "Endtrada de Áudio"),
        ("ID Server", "Servidor de ID"),
        ("Relay Server", "Servidor de Relay"),
        ("API Server", "Servidor da API"),
        ("invalid_http", "deve iniciar com http:// ou https://"),
        ("Invalid IP", "IP inválido"),
        ("id_change_tip", "Somente os caracteres a-z, A-Z, 0-9 e _ (sublinhado) são permitidos. A primeira letra deve ser a-z, A-Z. Comprimento entre 6 e 16."),
        ("Invalid format", "Formato inválido"),
        ("This function is turned off by the server", "Esta funcionalidade foi desligada pelo servidor"),
        ("Not available", "Indisponível"),
        ("Too frequent", "Muito frequente"),
        ("Cancel", "Cancelar"),
        ("Skip", "Pular"),
        ("Close", "Fechar"),
        ("Retry", "Tentar novamente"),
        ("OK", "OK"),
        ("Password Required", "Senha Necessária"),
        ("Please enter your password", "Por favor informe sua senha"),
        ("Remember password", "Lembrar senha"),
        ("Wrong Password", "Senha Incorreta"),
        ("Do you want to enter again?", "Você quer entrar novamente?"),
        ("Connection Error", "Erro de Conexão"),
        ("Error", "Erro"),
        ("Reset by the peer", "Reiniciado pelo par"),
        ("Connecting...", "Conectando..."),
        ("Connection in progress. Please wait.", "Conexão em progresso. Aguarde por favor."),
        ("Please try 1 minute later", "Por favor tente após 1 minuto"),
        ("Login Error", "Erro de Login"),
        ("Successful", "Sucesso"),
        ("Connected, waiting for image...", "Conectado. Aguardando pela imagem..."),
        ("Name", "Nome"),
        ("Type", "Tipo"),
        ("Modified", "Modificado"),
        ("Size", "Tamanho"),
        ("Show Hidden Files", "Mostrar Arquivos Ocultos"),
        ("Receive", "Receber"),
        ("Send", "Enviar"),
        ("Refresh File", "Atualizar Arquivo"),
        ("Local", "Local"),
        ("Remote", "Remoto"),
        ("Remote Computer", "Computador Remoto"),
        ("Local Computer", "Computador Local"),
        ("Confirm Delete", "Comfirmar Apagar"),
        ("Delete", "Apagar"),
        ("Properties", "Propriedades"),
        ("Multi Select", "Seleção Múltipla"),
        ("Empty Directory", "Diretório Vazio"),
        ("Not an empty directory", "Diretório não está vazio"),
        ("Are you sure you want to delete this file?", "Tem certeza que deseja apagar este arquivo?"),
        ("Are you sure you want to delete this empty directory?", "Tem certeza que deseja apagar este diretório vazio?"),
        ("Are you sure you want to delete the file of this directory?", "Tem certeza que deseja apagar este arquivo deste diretório?"),
        ("Do this for all conflicts", "Fazer isto para todos os conflitos"),
        ("This is irreversible!", "Isso é irreversível!"),
        ("Deleting", "Apagando"),
        ("files", "arquivos"),
        ("Waiting", "Aguardando"),
        ("Finished", "Completo"),
        ("Speed", "Velocidade"),
        ("Custom Image Quality", "Qualidade Visual Personalizada"),
        ("Privacy mode", "Modo privado"),
        ("Block user input", "Bloquear entrada de usuário"),
        ("Unblock user input", "Desbloquear entrada de usuário"),
        ("Adjust Window", "Ajustar Janela"),
        ("Original", "Original"),
        ("Shrink", "Reduzir"),
        ("Stretch", "Aumentar"),
        ("Good image quality", "Qualidade visual boa"),
        ("Balanced", "Balanceada"),
        ("Optimize reaction time", "Otimizar tempo de reação"),
        ("Custom", "Personalizado"),
        ("Show remote cursor", "Mostrar cursor remoto"),
        ("Disable clipboard", "Desabilitar área de transferência"),
        ("Lock after session end", "Bloquear após o fim da sessão"),
        ("Insert", "Inserir"),
        ("Insert Lock", "Inserir Trava"),
        ("Refresh", "Atualizar"),
        ("ID does not exist", "ID não existe"),
        ("Failed to connect to rendezvous server", "Falha ao conectar ao servidor de rendezvous"),
        ("Please try later", "Por favor tente mais tarde"),
        ("Remote desktop is offline", "Desktop remoto está offline"),
        ("Key mismatch", "Chaves incompatíveis"),
        ("Timeout", "Tempo esgotado"),
        ("Failed to connect to relay server", "Falha ao conectar ao servidor de relay"),
        ("Failed to connect via rendezvous server", "Falha ao conectar ao servidor de rendezvous"),
        ("Failed to connect via relay server", "Falha ao conectar através do servidor de relay"),
        ("Failed to make direct connection to remote desktop", "Falha ao fazer conexão direta ao desktop remoto"),
        ("Set Password", "Definir Senha"),
        ("OS Password", "Senha do SO"),
        ("install_tip", "Devido ao UAC, o ClientDesk não funciona corretamente como o lado remoto em alguns casos. Para evitar o UAC, por favor clique no botão abaixo para instalar o ClientDesk no sistema."),
        ("Click to upgrade", "Clique para fazer o upgrade"),
        ("Click to download", "Clique para baixar"),
        ("Click to update", "Clique para fazer o update"),
        ("Configure", "Configurar"),
        ("config_acc", "Para controlar seu Desktop remotamente, você precisa conceder ao ClientDesk permissões de \"Acessibilidade\"."),
        ("config_screen", "Para acessar seu Desktop remotamente, você precisa conceder ao ClientDesk permissões de \"Gravar a Tela\"/"),
        ("Installing ...", "Instalando ..."),
        ("Install", "Instalar"),
        ("Installation", "Instalação"),
        ("Installation Path", "Caminho da Instalação"),
        ("Create start menu shortcuts", "Criar atalhos no menu iniciar"),
        ("Create desktop icon", "Criar ícone na área de trabalho"),
        ("agreement_tip", "Ao iniciar a instalação, você concorda com o acordo de licença."),
        ("Accept and Install", "Aceitar e Instalar"),
        ("End-user license agreement", "Acordo de licença do usuário final"),
        ("Generating ...", "Gerando ..."),
        ("Your installation is lower version.", "Sua instalação é de uma versão menor."),
        ("not_close_tcp_tip", "Não feche esta janela enquanto estiver utilizando o túnel"),
        ("Listening ...", "Escutando ..."),
        ("Remote Host", "Host Remoto"),
        ("Remote Port", "Porta Remota"),
        ("Action", "Ação"),
        ("Add", "Adicionar"),
        ("Local Port", "Porta Local"),
        ("setup_server_tip", "Para uma conexão mais rápida, por favor configure seu próprio servidor"),
        ("Too short, at least 6 characters.", "Muito curto, pelo menos 6 caracteres."),
        ("The confirmation is not identical.", "A confirmação não é idêntica."),
        ("Permissions", "Permissões"),
        ("Accept", "Aceitar"),
        ("Dismiss", "Dispensar"),
        ("Disconnect", "Desconectar"),
        ("Allow using keyboard and mouse", "Permitir o uso de teclado e mouse"),
        ("Allow using clipboard", "Permitir o uso da área de transferência"),
        ("Allow hearing sound", "Permitir escutar som"),
        ("Allow file transfer", "Permitir transferência de arquivo"),
        ("File transfer", "Transferência de arquivo"),
        ("Connected", "Conectado"),
        ("Direct and encrypted connection", "Conexão direta e criptografada"),
        ("Relayed and encrypted connection", "Conexão via relay e criptografada"),
        ("Direct and unencrypted connection", "Conexão direta e não criptografada"),
        ("Relayed and unencrypted connection", "Conexão via relay e não criptografada"),
        ("Enter Remote ID", "Informe o ID Remoto"),
        ("Enter your password", "Informe sua senha"),
        ("Logging in...", "Fazendo Login..."),
        ("Enable RDP session sharing", "Habilitar compartilhamento de sessão RDP"),
        ("Auto Login", "Login Automático (Somente válido se você habilitou \"Bloquear após o fim da sessão\")"),
        ("Enable Direct IP Access", "Habilitar Acesso IP Direto"),
        ("Rename", "Renomear"),
        ("Space", "Espaõ"),
        ("Create Desktop Shortcut", "Criar Atalho na Área de Trabalho"),
        ("Change Path", "Alterar Caminho"),
        ("Create Folder", "Criar Diretório"),
        ("Please enter the folder name", "Por favor informe o nome do diretório"),
        ("Fix it", "Conserte"),
        ("Warning", "Aguardando"),
        ("Login screen using Wayland is not supported", "Tela de Login utilizando Wayland não é suportada"),
        ("Reboot required", "Reinicialização necessária"),
        ("Unsupported display server ", "Servidor de display não suportado"),
        ("x11 expected", "x11 esperado"),
        ("Port", "Porta"),
        ("Settings", "Configurações"),
        ("Username", "Nome de usuário"),
        ("Invalid port", "Porta inválida"),
        ("Closed manually by the peer", "Fechada manualmente pelo par"),
        ("Enable remote configuration modification", "Habilitar modificações de configuração remotas"),
        ("Run without install", "Executar sem instalar"),
        ("Always connected via relay", "Sempre conectado via relay"),
        ("Always connect via relay", "Sempre conectar via relay"),
        ("whitelist_tip", "Somente IPs na whitelist podem me acessar"),
        ("Login", "Login"),
        ("Logout", "Sair"),
        ("Tags", "Tags"),
        ("Search ID", "Buscar ID"),
        ("Current Wayland display server is not supported", "Servidor de display Wayland atual não é suportado"),
        ("whitelist_sep", "Separado por vírcula, ponto-e-vírgula, espaços ou nova linha"),
        ("Add ID", "Adicionar ID"),
        ("Add Tag", "Adicionar Tag"),
        ("Unselect all tags", "Desselecionar todas as tags"),
        ("Network error", "Erro de rede"),
        ("Username missed", "Nome de usuário faltante"),
        ("Password missed", "Senha faltante"),
        ("Wrong credentials", "Nome de usuário ou senha incorretos"),
        ("Edit Tag", "Editar Tag"),
        ("Unremember Password", "Esquecer Senha"),
        ("Favorites", "Favoritos"),
        ("Add to Favorites", "Adicionar aos Favoritos"),
        ("Remove from Favorites", "Remover dos Favoritos"),
        ("Empty", "Vazio"),
        ("Invalid folder name", "Nome de diretório inválido"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Nome de anfitrião"),
        ("Discovered", "Descoberto"),
        ("install_daemon_tip", "Para inicialização junto ao sistema, você deve instalar o serviço de sistema."),
        ("Remote ID", "ID Remoto"),
        ("Paste", "Colar"),
        ("Paste here?", "Colar aqui?"),
        ("Are you sure to close the connection?", "Tem certeza que deseja fechar a conexão?"),
        ("Download new version", "Baixar nova versão"),
        ("Touch mode", "Modo toque"),
        ("Mouse mode", "Modo mouse"),
        ("One-Finger Tap", "Toque com um dedo"),
        ("Left Mouse", "Botão esquerdo do mouse"),
        ("One-Long Tap", "Um toque longo"),
        ("Two-Finger Tap", "Toque com dois dedos"),
        ("Right Mouse", "Botão direito do mouse"),
        ("One-Finger Move", "Mover com um dedo"),
        ("Double Tap & Move", "Toque duplo & mover"),
        ("Mouse Drag", "Arrastar com o mouse"),
        ("Two-Finger vertically", "Dois dedos verticalmente"),
        ("Mouse Wheel", "Roda do Mouse"),
        ("Two-Finger Move", "Mover com dois dedos"),
        ("Canvas Move", "Mover Tela"),
        ("Pinch to Zoom", "Beliscar para Zoom"),
        ("Canvas Zoom", "Zoom na Tela"),
        ("Reset canvas", "Reiniciar tela"),
        ("No permission of file transfer", "Sem permissões de transferência de arquivo"),
        ("Note", "Nota"),
        ("Connection", "Conexão"),
        ("Share Screen", "Compartilhar Tela"),
        ("CLOSE", "FECHAR"),
        ("OPEN", "ABRIR"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "itens"),
        ("Selected", "Selecionado"),
        ("Screen Capture", "Captura de Tela"),
        ("Input Control", "Controlo de Entrada"),
        ("File Transfer", "Transferência de Arquivo"),
        ("Audio Capture", "Captura de Áudio"),
        ("File Connection", "Conexão de Arquivo"),
        ("Screen Connection", "Conexão de Tela"),
        ("Do you accept?", "Você aceita?"),
        ("Open System Setting", "Abrir Configurações do Sistema"),
        ("How to get Android input permission?", "Como habilitar a permissão de entrada do Android?"),
        ("android_input_permission_tip1", "Para que um dispositivo remoto controle seu dispositivo Android via mouse ou toque, você precisa permitir que o ClientDesk use o serviço \"Acessibilidade\"."),
        ("android_input_permission_tip2", "Por favor vá para a próxima página de configuração do sistema, encontre e entre [Serviços Instalados], HABILITE o serviço [ClientDesk Input]."),
        ("android_new_connection_tip", "Nova requisição de controle recebida, solicita o controle de seu dispositivo atual."),
        ("android_service_will_start_tip", "Habilitar a Captura de Tela irá automaticamente inicalizar o serviço, permitindo que outros dispositivos solicitem uma conexão deste dispositivo."),
        ("android_stop_service_tip", "Fechar o serviço irá automaticamente fechar todas as conexões estabelecidas."),
        ("android_version_audio_tip", "A versão atual do Android não suporta captura de áudio, por favor atualize para o Android 10 ou maior."),
        ("android_start_service_tip", "Toque [Iniciar Serviço] ou ABRA a permissão [Captura de Tela] para iniciar o serviço de compartilhamento de tela."),
        ("Account", "Conta"),
        ("Quit", "Saída"),
        ("Help", "Ajuda"),
    ].iter().cloned().collect();
}
