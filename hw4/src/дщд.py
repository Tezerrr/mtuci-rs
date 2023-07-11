class Vector():
    def __init__(self):
        self.data = list()

    def new(self):
        return Vector
    def push(self, item):
        self.data.append(item)

    def pop(self):
        try:
            return self.data[-1]
        except:
            return None

    def get(self, number):
        try:
            return self.data[number]
        except:
            return None

    def remove(self, number):
        try:
            return self.data.remove(number)
        except:
            return None
    def insert(self, key, value):
        try:
            self.data[key] = value
        except:
            pass
