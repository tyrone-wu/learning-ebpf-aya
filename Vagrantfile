# -*- mode: ruby -*-
# vi: set ft=ruby :

required_plugins = %w( vagrant-vbguest vagrant-reload )
required_plugins.each do |plugin|
    exec "vagrant plugin install #{plugin};vagrant #{ARGV.join(" ")}" unless Vagrant.has_plugin? plugin || ARGV[0] == 'plugin'
end

Vagrant.configure("2") do |config|
  config.vm.define "aya-sandbox" do |c|
    c.vm.box = "generic/ubuntu2204"
    c.vbguest.auto_update = false
    c.vm.synced_folder ".", "/vagrant", type: "smb"

    c.vm.provider "virtualbox" do |v|
      v.name = "aya-sandbox"
      v.memory = 6144
      v.cpus = 4
    end

    c.vm.provision "shell", privileged: false, inline: <<-SHELL
      # Install prereq
      wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
      sudo apt-add-repository -y "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-16 main"
      sudo apt-get update -y
      # llvm 16 for aya
      sudo DEBIAN_FRONTEND=noninteractive apt-get install -y llvm-16-dev libclang-16-dev libpolly-16-dev
      # bpf-linker
      sudo DEBIAN_FRONTEND=noninteractive apt-get install -y build-essential libfontconfig1-dev
      # bpftool
      sudo DEBIAN_FRONTEND=noninteractive apt-get install -y libelf-dev binutils-dev libcap-dev
      # cargo-generate
      sudo DEBIAN_FRONTEND=noninteractive apt-get install -y libssl-dev
    
      # Install bpftool
      git clone --recurse-submodules https://github.com/libbpf/bpftool.git
      cd bpftool/src/ && sudo make install
      #sudo rm -rf ~/bpftool/
      
      # Install rustup
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      source "$HOME/.cargo/env"
      
      # Setup dev environment for aya: https://aya-rs.dev/book/start/development/
      rustup install stable
      rustup toolchain install nightly --component rust-src
      
      # Install bpf linker
      cargo install bpf-linker --no-default-features
      
      # For development convenience
      cargo install cargo-generate
    SHELL
    
    # Reboot vm
    c.vm.provision :reload
  end
end
