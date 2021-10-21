with import (import ./default.nix).inputs.nixpkgs {}; mkShell { 

    buildInputs = [ 
        nixFlakes
        (writeScriptBin "ci-build" ''
          nix build --experimental-features 'nix-command flakes'
        '')
    ];

}

