import requests

class InteractiveBrokers:
    def __init__(self):
        self.base_url = "https://qa.interactivebrokers.com"
        self.token = None
        # /oauth2/api/v1/token
        pass
    def get_token(self):
        url = f"{self.base_url}/oauth2/api/v1/token"
        data = {
            "gran_type": "",
            "client_assertion": "",
            "client_assertion_type": "",
            "scope": ""
        }
        response = requests.post(url, data=data)
        print(response.json())

if __name__ == '__main__':

    ib = InteractiveBrokers()
    ib.get_token()
