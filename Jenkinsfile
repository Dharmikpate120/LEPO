pipeline{
    agent any
    tools{
        dockerTool  "docker"
    }
    stages{
        stage("build"){
            steps{
                sh 'docker stop lepo-backend:latest || true'
                sh 'docker rmi lepo-backend:latest || true'
                sh 'docker build . -t lepo-backend:latest'
            }
        }
        
        stage("test"){
            steps{
                echo "testing"
            }
        }
        stage("deploy"){
            steps{
                sh 'docker run -d --name lepo-backend --rm -p 8000:8000 lepo-backend:latest'
            }
        }
    }
}