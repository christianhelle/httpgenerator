using Atc.Test;
using FluentAssertions;
using HttpGenerator.Core;

namespace HttpGenerator.Tests;

public class SerializerTests
{
    [Theory, AutoNSubstituteData]
    public void Can_Serialize_GeneratorSettings(
        GeneratorSettings settings)
    {
        Serializer
            .Serialize(settings)
            .Should()
            .NotBeNullOrWhiteSpace();
    }

    [Theory, AutoNSubstituteData]
    public void Can_Deserialize_GeneratorSettings(
        GeneratorSettings settings)
    {
        var json = Serializer.Serialize(settings);
        Serializer
            .Deserialize<GeneratorSettings>(json)
            .Should()
            .BeEquivalentTo(settings);
    }

    [Theory, AutoNSubstituteData]
    public void Deserialize_Is_Case_Insensitive(
        GeneratorSettings settings)
    {
        var json = Serializer.Serialize(settings);
        foreach (var property in typeof(GeneratorSettings).GetProperties())
        {
            var jsonProperty = "\"" + property.Name + "\"";
            json = json.Replace(
                jsonProperty, 
                jsonProperty.ToUpperInvariant());
        }

        Serializer
            .Deserialize<GeneratorSettings>(json)
            .Should()
            .BeEquivalentTo(settings);
    }
}