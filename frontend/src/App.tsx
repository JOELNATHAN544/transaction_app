import { MainLayout } from './components/layout/MainLayout'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './components/ui/card'
import { Button } from './components/ui/button'

function App() {
  return (
    <MainLayout>
      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {/* Example Product Card */}
        <Card>
          <CardHeader>
            <CardTitle>Digital Product</CardTitle>
            <CardDescription>High-quality digital content</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">$29.99</p>
            <p className="text-sm text-muted-foreground">
              Get access to premium digital content with lifetime updates.
            </p>
          </CardContent>
          <CardFooter>
            <Button className="w-full">Purchase Now</Button>
          </CardFooter>
        </Card>

        {/* Add more product cards here */}
      </div>
    </MainLayout>
  )
}

export default App
