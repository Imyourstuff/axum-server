
def gv

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
		stage("init") {
					steps {
						script {
							gv = load "script.groovy"
						}
					}
				}
		stage("build") {
			steps {
				script {
					gv.buildApp()
				}
			}
		}

		stage("test") {
			when {
				expression {
					params.ExecuteTests
				}
			}
			steps {
				script {
					gv.testApp()
				}
			}
		}
		
		stage("deploy") {
			steps {
				withCredentials([
					usernamePassword(credentialsId: 'docker-hub-repo', usernameVariable: 'USER', passwordVariable: 'PWD')
				]) {
					sh "echo $USER $PWD"
				}
				script {
					gv.deployApp()
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
