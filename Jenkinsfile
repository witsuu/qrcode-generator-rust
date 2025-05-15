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
                    // Debug 1: Tampilkan semua parameter
                    echo "All build parameters: ${params}"
                    
                    // Debug 2: Tampilkan penyebab build
                    echo "Build causes: ${currentBuild.getBuildCauses()}"
                    
                    // Debug 3: Coba baca payload manual
                    try {
                        def payload = readJSON text: currentBuild.getBuildCauses()[0].shortDescription
                        echo "Raw payload: ${payload}"
                        env.TAG_NAME = payload.release?.tag_name
                    } catch(e) {
                        echo "Cannot parse payload: ${e.message}"
                    }
                    
                    // Validasi akhir
                    if (!env.TAG_NAME?.trim()) {
                        error """
                        ❌ TAG_NAME not found. Possible causes:
                        1. GitHub webhook misconfigured
                        2. Generic Webhook Trigger plugin not extracting properly
                        3. Payload format mismatch
                        Current env: ${env.getEnvironment()}
                        """
                    }
                    echo "✅ Using TAG_NAME: ${env.TAG_NAME}"
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