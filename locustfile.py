
from locust import HttpUser, task, between
from faker import Faker

# Create a Faker instance to generate fake data
fake = Faker()

class WebsiteUser(HttpUser):
    host = "http://localhost:5000"
    wait_time = between(1, 5)  # Wait time between tasks in seconds

    @task
    def send_log(self):
        # Generate a random IPv4 address
        ip_address = fake.ipv4()

        # Define the JSON payload
        payload = [{"ip": ip_address}]

        # Send the POST request
        self.client.post("/logs", json=payload)
