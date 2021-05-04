using System.Windows;

namespace calculator
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow
    {
        public MainWindow()
        {
            InitializeComponent();
        }

        private void btn0_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "0";
        }
        
        private void btn1_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "1";
        }

        private void btn2_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "2";
        }

        private void btn3_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "3";
        }

        private void btn4_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "4";
        }

        private void btn5_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "5";
        }

        private void btn6_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "6";
        }

        private void btn7_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "7";
        }

        private void btn8_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "8";
        }

        private void btn9_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "9";
        }

        private void clear_click(object sender, RoutedEventArgs e)
        {
            formula.Text = "";
        }

        private void left_paren_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "(";
        }

        private void right_paren_click(object sender, RoutedEventArgs e)
        {
            formula.Text += ")";
        }

        private void backspace_click(object sender, RoutedEventArgs e)
        {
            var len = formula.Text.Length;
            if (len == 0) return;
            formula.Text = formula.Text.Remove(len - 1, 1);
        }

        private void div_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "/";
        }

        private void mul_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "*";
        }

        private void sub_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "-";
        }

        private void dot_click(object sender, RoutedEventArgs e)
        {
            formula.Text += ".";
        }

        private void equal_click(object sender, RoutedEventArgs e)
        {
            
        }

        private void add_click(object sender, RoutedEventArgs e)
        {
            formula.Text += "+";
        }
    }
}