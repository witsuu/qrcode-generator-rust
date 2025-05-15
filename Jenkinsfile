pipeline {
    agent any

    // Environment variables global
    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo"
        RUSTUP_HOME = "${WORKSPACE}/.rustup"
        PATH = "${WORKSPACE}/.cargo/bin:${PATH}"
    }

    stages {
        // Stage 1: Debugging - Capture payload
        stage('Capture Webhook Data') {
            steps {
                script {
                    // Log semua environment variables
                    echo "All env variables:\n${env}"
                    
                    // Verifikasi TAG_NAME dari webhook
                    if (!env.TAG_NAME?.trim()) {
                        error "❌ TAG_NAME not found in webhook payload. Ensure GitHub webhook is properly configured with release.tag_name"
                    }
                    echo "✅ Received TAG_NAME from webhook: ${env.TAG_NAME}"
                }
            }
        }

        // Stage 2: Setup Rust
        stage('Setup Rust') {
            steps {
                sh '''#!/bin/bash -xe
                    curl --fail https://sh.rustup.rs -sSf | sh -s -- -y
                    source "${CARGO_HOME}/env"
                    rustc --version
                    cargo --version
                '''
            }
        }

        // Stage 3: Clone repository
        stage('Clone Tag') {
            steps {
                echo "🔄 Cloning tag: ${env.TAG_NAME}"
                checkout([
                    $class: 'GitSCM',
                    branches: [[name: "refs/tags/${env.TAG_NAME}"]],
                    userRemoteConfigs: [[
                        url: 'https://github.com/witsuu/qrcode-generator-rust.git',
                    ]],
                    extensions: [[
                        $class: 'CloneOption',
                        depth: 1, //Shallow clone
                        noTags: false
                    ]]
                ])
            }
        }

        // Stage 4: Build
        stage('Build Release') {
            steps {
                sh '''#!/bin/bash -xe
                    source "${CARGO_HOME}/env"
                    cargo build --release
                '''
            }
        }
    }

    post {
        success {
            echo "✅ Successfully built ${env.TAG_NAME}"
            // Tambahkan upload artifact jika perlu
        }
        failure {
            echo "❌ Failed to build ${env.TAG_NAME}"
        }
    }
}