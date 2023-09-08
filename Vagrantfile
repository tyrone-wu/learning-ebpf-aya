# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.define "aya-sandbox" do |c|
    c.vm.box = "generic/ubuntu2204"
    c.vbguest.auto_update = false
    c.vm.synced_folder ".", "/vagrant", type: "virtualbox"

    c.vm.provider "virtualbox" do |v|
      v.name = "aya-sandbox"
      v.memory = 6144
      v.cpus = 4
    end

    c.vm.provision "shell", privileged: false, inline: <<-SHELL
      # Install prereq and llvm 16
      wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
      sudo apt-add-repository -y "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-16 main"
      sudo apt-get update -y
      sudo DEBIAN_FRONTEND=noninteractive apt-get install -y build-essential libfontconfig1-dev pkg-config libssl-dev llvm-16-dev libclang-16-dev libpolly-16-dev
      sudo systemctl restart multipathd.service packagekit.service
    
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
  end
end
