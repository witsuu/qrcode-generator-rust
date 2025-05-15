properties([
    parameters([
        string(name: 'TAG_NAME', defaultValue: '', description: 'GitHub Release tag name')
    ])
])


pipeline {
    agent any

    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo"
        RUSTUP_HOME = "${WORKSPACE}/.rustup"
        PATH = "${WORKSPACE}/.cargo/bin:${PATH}"
    }

    stages {
        stage('Debug TAG_NAME') {
            steps {
                echo "TAG_NAME is: ${env.TAG_NAME}"
            }
        }
        stage('Setup Rust') {
            steps {
                sh '''
                    curl https://sh.rustup.rs -sSf | sh -s -- -y
                    . ${WORKSPACE}/.cargo/env
                    rustc --version
                    cargo --version
                '''
            }
        }

        stage('Clone Tag') {
            steps {
                echo "🚀 Cloning release tag: ${env.TAG_NAME}"
                checkout([$class: 'GitSCM',
                    branches: [[name: "refs/tags/${env.TAG_NAME}"]],
                    userRemoteConfigs: [[url: 'https://github.com/witsuu/qrcode-generator-rust.git']]
                ])
            }
        }

        stage('Build Release') {
            steps {
                sh '''
                    . ${WORKSPACE}/.cargo/env
                    cargo build --release
                '''
            }
        }
    }
}
