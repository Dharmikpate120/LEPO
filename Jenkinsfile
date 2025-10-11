pipeline{
    agent any
    tools{
        docker  "docker"
    }
    stages{
        stage("build"){
            step{
                sh 'docker stop lepo-backend:latest || true'
                sh 'docker rmi lepo-backend:latest || true'
                sh 'docker build . -t lepo-backend:latest'
            }
        }
        stage("test"){
            step{
                echo "testing"
            }
        }
        stage("deploy"){
            step{
                sh 'docker run -p 8000:8000 lepo-backend:latest'
            }
        }
    }
}