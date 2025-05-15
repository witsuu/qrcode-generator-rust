// ✨ Deklarasi parameter dari webhook GitHub
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
        stage('Validate TAG_NAME') {
            steps {
                script {
                    if (!env.TAG_NAME?.trim()) {
                        error "❌ TAG_NAME is missing. Make sure this job is triggered by a GitHub Release webhook."
                    }
                    echo "✅ Received TAG_NAME: ${env.TAG_NAME}"
                }
            }
        }

        stage('Setup Rust') {
            steps {
                sh '''#!/bin/bash
                    curl https://sh.rustup.rs -sSf | sh -s -- -y
                    . ${WORKSPACE}/.cargo/env
                    rustc --version
                    cargo --version
                '''
            }
        }

        stage('Clone Tag') {
            steps {
                echo "🔄 Cloning release tag: ${env.TAG_NAME}"
                checkout([$class: 'GitSCM',
                    branches: [[name: "refs/tags/${env.TAG_NAME}"]],
                    userRemoteConfigs: [[url: 'https://github.com/witsuu/qrcode-generator-rust.git']]
                ])
            }
        }

        stage('Build Release') {
            steps {
                sh '''#!/bin/bash
                    . ${WORKSPACE}/.cargo/env
                    cargo build --release
                '''
            }
        }

        // Optional: tambahkan test atau upload ke GitHub Release/binary server
    }

    post {
        success {
            echo "✅ Build completed successfully for ${env.TAG_NAME}"
        }
        failure {
            echo "❌ Build failed for ${env.TAG_NAME}"
        }
    }
}
