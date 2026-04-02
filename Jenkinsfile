
pipeline {
	
	agent any
	// Gradle Maven Jdk only lol...
	//tools {
	//	maven "maven3.9"
	//}
	parameters {
		string(name: 'VERSION', defaultValue: '', description: 'version to deploy on prod')
		choice(name: 'VERSION', choices: ['1.1.0', '1.2.0', '1.3.0'], description: '')
		booleanParam(name: 'ExecuteTests', defaultValue: true, description: '')
	}
	environment {
		NEW_VERSION = '1.3.0'
		SERVER_CREDENTIALS = credentials('docker-hub-repo')
	}
	stages {

		stage("build") {
			steps {
				echo 'Building the application'
				echo "Building version ${NEW_VERSION}"
			}
		}

		stage("test") {
			when {
				expression {
					params.ExecuteTests
				}
			}
			steps {
				echo "Testing the application"
			}
		}
		
		stage("deploy") {
			steps {
				withCredentials([
					usernamePassword(credentialsId: 'docker-hub-repo', usernameVariable: 'USER', passwordVariable: 'PWD')
				]) {
					sh "echo $USER $PWD"
				}
			echo "Deploying successfull! Version: ${params.VERSION}"
			}
		}
	
	}
//	post {
//		always {
//			sh 
//		}
//		failure {}
//	}

}
